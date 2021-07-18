use super::models::{
    Extended, HybridAssociation, HybridMod, IncubatedItem, Influence, Item, Mod, Property,
    PropertyTypeDb, RemoveItems, Socket, SocketedItem, SplittedItem, Subcategory, UltimatumMod,
    NewLatestStash
};
use super::TypedConnectionPool;
use crate::domain::item::Item as DomainItem;
use crate::ports::outbound::public_stash_retriever::PublicStashData;
use crate::ports::outbound::repository::{LatestStashId, RepositoryError};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::BelongingToDsl;
use itertools::Itertools;
use std::{
    collections::HashMap,
    convert::{From, TryFrom},
};
use tracing::{event, instrument, Level};
use uuid::Uuid;

macro_rules! collect_val {
    ($v:expr, $field:tt) => {
        $v.iter()
            .map(|el| &el.$field)
            .filter_map(|el| el.as_ref())
            .flatten()
    };
}

macro_rules! insert_val {
    ($v:expr, $field:tt, $table:tt, $conn:expr) => {
        for mod_ in collect_val!($v, $field) {
            // TODO: check why sometimes we trying to insert already presented values
            diesel::replace_into($table).values(mod_).execute(&$conn)?;
        }
    };
}

macro_rules! collect_val2 {
    ($v:expr, $field:tt) => {
        $v.iter().map(|el| &el.$field).filter_map(|el| el.as_ref())
    };
}

macro_rules! insert_val2 {
    ($v:expr, $field:tt, $table:tt, $conn:expr) => {
        for mod_ in collect_val2!($v, $field) {
            // TODO: check why sometimes we trying to insert already presented values
            diesel::replace_into($table).values(mod_).execute(&$conn)?;
        }
    };
}

#[derive(Clone)]
pub struct DieselItemRepository {
    conn: TypedConnectionPool,
}

use diesel::expression::{AppearsOnTable, Expression, NonAggregate};
use diesel::query_builder::{QueryFragment, QueryId};
use diesel::query_dsl::RunQueryDsl;
use diesel::sql_types::Bool;
use diesel::sqlite::Sqlite;

impl DieselItemRepository {
    pub fn new(conn: TypedConnectionPool) -> Result<DieselItemRepository, RepositoryError> {
        Ok(DieselItemRepository { conn })
    }

    #[instrument(err, skip(self, query))]
    fn get_items_by_query<'a, U>(&self, query: U) -> Result<Vec<DomainItem>, RepositoryError>
    where
        U: Expression<SqlType = Bool>
            + NonAggregate
            + AppearsOnTable<crate::schema::items::table>
            + QueryFragment<Sqlite>
            + QueryId,
    {
        use crate::schema::hybrid_mods::dsl as hybrid_mods_dsl;
        use crate::schema::items::dsl::items as items_table;
        use crate::schema::property_types::dsl as property_types_dsl;
        use itertools::izip;

        let conn = self.conn.get()?;

        let items = items_table.filter(query).load::<Item>(&conn)?;

        let influences = Influence::belonging_to(&items)
            .load::<Influence>(&conn)?
            .grouped_by(&items);
        let mods = Mod::belonging_to(&items)
            .load::<Mod>(&conn)?
            .grouped_by(&items);
        let extended = Extended::belonging_to(&items)
            .load::<Extended>(&conn)?
            .grouped_by(&items);

        let hybrid = HybridAssociation::belonging_to(&items).load::<HybridAssociation>(&conn)?;
        let hybrid_mods = hybrid_mods_dsl::hybrid_mods
            .filter(hybrid_mods_dsl::id.eq_any(hybrid.iter().map(|e| &e.hybrid_id)))
            .load::<HybridMod>(&conn)?;

        let hybrid_mods = hybrid_mods
            .into_iter()
            .map(|el| (el.id.clone(), el))
            .collect::<HashMap<String, HybridMod>>();

        let hybrid = hybrid.grouped_by(&items);
        let hybrid = hybrid
            .into_iter()
            .map(|el| {
                el.into_iter()
                    .map(|inner_el| hybrid_mods.get(&inner_el.hybrid_id).cloned().unwrap())
                    .collect::<Vec<_>>()
                    .first()
                    .cloned()
            })
            .collect::<Vec<Option<_>>>();

        let incubated = IncubatedItem::belonging_to(&items)
            .load::<IncubatedItem>(&conn)?
            .grouped_by(&items);
        let ultimatum = UltimatumMod::belonging_to(&items)
            .load::<UltimatumMod>(&conn)?
            .grouped_by(&items);
        let socket = Socket::belonging_to(&items)
            .load::<Socket>(&conn)?
            .grouped_by(&items);
        let socketed = SocketedItem::belonging_to(&items)
            .load::<SocketedItem>(&conn)?
            .grouped_by(&items);

        let properties = Property::belonging_to(&items).load::<Property>(&conn)?;
        let property_types = property_types_dsl::property_types
            .filter(property_types_dsl::id.eq_any(properties.iter().map(|e| &e.property_id)))
            .load::<PropertyTypeDb>(&conn)?;

        let property_types = property_types
            .into_iter()
            .map(|el| (el.id.clone(), el))
            .collect::<HashMap<String, PropertyTypeDb>>();

        let properties = properties.grouped_by(&items);
        let properties = properties
            .into_iter()
            .map(|el| {
                el.into_iter()
                    .map(|inner_el| property_types.get(&inner_el.property_id).cloned().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();

        let subcategories = Subcategory::belonging_to(&items)
            .load::<Subcategory>(&conn)?
            .grouped_by(&items);

        let data = izip!(
            items,
            influences,
            mods,
            extended,
            hybrid,
            incubated,
            ultimatum,
            socket,
            socketed,
            properties,
            subcategories
        )
        .map(|v| DomainItem::from(v))
        .collect::<Vec<_>>();

        Ok(data)
    }

    #[instrument(err, skip(self))]
    pub fn get_items_by_basetype(
        &self,
        base_type_: &str,
    ) -> Result<Vec<DomainItem>, RepositoryError> {
        use crate::schema::items::dsl::*;

        self.get_items_by_query(base_type.eq(base_type_))
    }

    #[instrument(err, skip(self))]
    pub fn get_items_by_ids(&self, ids: Vec<String>) -> Result<Vec<DomainItem>, RepositoryError> {
        use crate::schema::items::dsl::*;

        self.get_items_by_query(id.eq_any(&ids))
    }

    fn _get_raw_items(
        &self,
        account_name_: &str,
        stash_id_: &str,
    ) -> Result<Vec<Item>, RepositoryError> {
        use crate::schema::items::dsl::*;

        let conn = self.conn.get()?;

        Ok(items
            .filter(account_name.eq(account_name_).and(stash_id.eq(stash_id_)))
            .load::<Item>(&conn)?)
    }

    #[instrument(err, skip(self))]
    pub fn get_stash_id(&self) -> Result<LatestStashId, RepositoryError> {
        use crate::schema::latest_stash_id::dsl::*;

        let conn = self.conn.get()?;

        let v = latest_stash_id.first::<LatestStashId>(&conn)?;
        Ok(v)
    }
    #[instrument(err, skip(self))]
    pub fn set_stash_id(&self, id_: &str) -> Result<(), RepositoryError> {
        use crate::schema::latest_stash_id::dsl::*;

        let conn = self.conn.get()?;

        let latest_stash = NewLatestStash { id: id_.to_owned() };

        match latest_stash_id.first::<LatestStashId>(&conn) {
            Err(e) => match e {
                DieselError::NotFound => {
                    diesel::insert_into(latest_stash_id)
                        .values(&latest_stash)
                        .execute(&conn)?;
                }
                _ => {
                    event!(Level::ERROR, "{}", e);
                    return Err(RepositoryError::Db);
                }
            },
            Ok(_) => {
                diesel::update(latest_stash_id)
                    .set(id.eq(id_))
                    .execute(&conn)?;
            }
        };

        Ok(())
    }

    #[instrument(err, skip(self, public_data))]
    pub fn insert_raw_item(&self, public_data: &PublicStashData) -> Result<(), RepositoryError> {
        use crate::schema::items::dsl::*;
        use crate::schema::{
            extended::dsl::extended as extended_table, hybrid_mods::dsl as hybrid_mods_dsl,
            hybrids::dsl::hybrids as hybrids_table,
            incubated_item::dsl::incubated_item as incubated_item_table,
            influences::dsl::influences as influences_table, mods::dsl::mods as mods_table,
            properties::dsl::properties as properties_table,
            property_types::dsl as property_types_dsl, sockets::dsl::sockets as sockets_table,
            subcategories::dsl::subcategories as subcategories_table,
            ultimatum_mods::dsl::ultimatum_mods as ultimatum_mods_table,
        };

        let new_item_info: HashMap<String, Vec<SplittedItem>> = public_data
            .stashes
            .iter()
            .map(|v| {
                v.items
                    .iter()
                    .map(|i| match SplittedItem::try_from(i.clone()) {
                        Ok(mut item) => {
                            item.item.account_id = v.id.clone();
                            item.item.account_name = v.account_name.as_ref().cloned().unwrap();
                            item.item.stash_id = v.stash.as_ref().cloned().unwrap();
                            Some(item)
                        }
                        Err(_) => {
                            event!(
                                Level::WARN,
                                "skipping {:?} item because cant generate entity",
                                i
                            );
                            None
                        }
                    })
                    .filter_map(|i| i)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .into_group_map_by(|el| el.item.account_id.clone());

        let conn = self.conn.get()?;
        // TODO: switch to item-by-item inserts
        conn.transaction::<_, RepositoryError, _>(|| {
            // TODO: need somehow run set_stash_id method in this transaction
            let latest_stash = NewLatestStash {
                id: public_data.next_change_id.clone(),
            };

            {
                use crate::schema::latest_stash_id::dsl::*;
                match latest_stash_id.first::<LatestStashId>(&conn) {
                    Err(e) => match e {
                        DieselError::NotFound => {
                            diesel::insert_into(latest_stash_id)
                                .values(&latest_stash)
                                .execute(&conn)?;
                        }
                        _ => {
                            event!(Level::ERROR, "{}", e);
                            return Err(RepositoryError::Db);
                        }
                    },
                    Ok(_) => {
                        diesel::update(latest_stash_id)
                            .set(id.eq(&latest_stash.id))
                            .execute(&conn)?;
                    }
                };
            }

            for (k, v) in &new_item_info {
                if v.len() == 0 {
                    diesel::delete(items.filter(account_id.eq(&k))).execute(&conn)?;
                    continue;
                }
                let insert_items: Vec<_> = v.iter().map(|v| &v.item).collect();
                let delete_items: Vec<_> = v
                    .iter()
                    .map(|v| RemoveItems {
                        account_name: &v.item.account_name,
                        stash_id: &v.item.stash_id,
                    })
                    .collect();

                // TODO: somehow use Identifiable or smth else to simplify delete query
                for i in &delete_items {
                    diesel::delete(
                        items.filter(account_name.eq(i.account_name).and(stash_id.eq(i.stash_id))),
                    )
                    .execute(&conn)?;
                }

                event!(Level::DEBUG, "inserting items");
                for i in insert_items {
                    diesel::insert_into(items).values(i).execute(&conn)?;
                }

                event!(
                    Level::DEBUG,
                    "inserting mods, subcats, props, sockets, ultimatums"
                );

                insert_val!(v, mods, mods_table, conn);
                insert_val!(v, subcategories, subcategories_table, conn);
                insert_val!(v, sockets, sockets_table, conn);
                insert_val!(v, ultimatum, ultimatum_mods_table, conn);

                event!(
                    Level::DEBUG,
                    "inserting incubated, hybrid, extended, influences"
                );

                insert_val2!(v, incubated, incubated_item_table, conn);
                insert_val2!(v, extended, extended_table, conn);
                insert_val2!(v, influence, influences_table, conn);
                for mods in collect_val2!(v, props) {
                    for mod_ in mods {
                        let new_prop = PropertyTypeDb {
                            id: Uuid::new_v4().to_hyphenated().to_string(),
                            name: mod_.name.clone(),
                            property_type: mod_.property_type,
                        };

                        let id_mod = match property_types_dsl::property_types
                            .filter(
                                property_types_dsl::property_type
                                    .eq(mod_.property_type)
                                    .and(property_types_dsl::name.eq(&mod_.name)),
                            )
                            .select(property_types_dsl::id)
                            .first::<String>(&conn)
                        {
                            Err(e) => match e {
                                DieselError::NotFound => {
                                    diesel::insert_into(property_types_dsl::property_types)
                                        .values(&new_prop)
                                        .execute(&conn)?;
                                    new_prop.id
                                }
                                _ => {
                                    event!(Level::ERROR, "cant find prop: {:?}", e);
                                    return Err(RepositoryError::Db);
                                }
                            },
                            Ok(k) => k,
                        };

                        diesel::replace_into(properties_table)
                            .values(&Property {
                                item_id: mod_.item_id.clone(),
                                property_id: id_mod,
                                progress: mod_.progress,
                                suffix: mod_.suffix.clone(),
                                type_: mod_.type_,
                                value: mod_.value.clone(),
                                value_type: mod_.value_type,
                            })
                            .execute(&conn)?;
                    }
                }

                for mods in collect_val2!(v, hybrid) {
                    let new_mod = HybridMod {
                        id: Uuid::new_v4().to_hyphenated().to_string(),
                        is_vaal_gem: mods.is_vaal_gem,
                        base_type_name: mods.base_type_name.clone(),
                        sec_descr_text: mods.sec_descr_text.clone(),
                    };

                    let id_mod = match hybrid_mods_dsl::hybrid_mods
                        .filter(
                            hybrid_mods_dsl::is_vaal_gem
                                .eq(&new_mod.is_vaal_gem)
                                .and(hybrid_mods_dsl::base_type_name.eq(&new_mod.base_type_name)),
                        )
                        .select(hybrid_mods_dsl::id)
                        .first::<String>(&conn)
                    {
                        Err(e) => match e {
                            DieselError::NotFound => {
                                diesel::insert_into(hybrid_mods_dsl::hybrid_mods)
                                    .values(&new_mod)
                                    .execute(&conn)?;
                                new_mod.id
                            }
                            _ => {
                                event!(Level::ERROR, "cant find hybrid: {:?}", e);
                                return Err(RepositoryError::Db);
                            }
                        },
                        Ok(k) => k,
                    };

                    event!(
                        Level::DEBUG,
                        "generated hybrid id {} for {:?}",
                        id_mod,
                        mods
                    );

                    diesel::replace_into(hybrids_table)
                        .values(&HybridAssociation {
                            hybrid_id: id_mod.clone(),
                            item_id: mods.item_id.clone(),
                        })
                        .execute(&conn)?;
                }
            }

            Ok(())
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::DieselItemRepository;
    use super::TypedConnectionPool;
    use crate::ports::outbound::public_stash_retriever::PublicStashData;
    use diesel::r2d2::{ConnectionManager, Pool};
    use std::path::PathBuf;
    use temp_file::{empty, TempFile};
    use tracing_subscriber;

    lazy_static::lazy_static! {
        static ref TRACING_EXEC: i32 = {
            tracing_subscriber::fmt::init();
            1
        };
    }

    const PUBLIC_STASH_DATA: &str = include_str!("public-stash-tabs.json");

    embed_migrations!("migrations");

    fn prepare_db() -> Result<(TypedConnectionPool, TempFile), anyhow::Error> {
        let _ = TRACING_EXEC;
        let f = empty();

        let pool = Pool::new(ConnectionManager::new(f.path().to_str().unwrap()))?;

        {
            let conn = pool.get()?;
            embedded_migrations::run(&conn)?;
        }

        Ok((pool, f))
    }

    fn _copy_file(tmp: &TempFile, dst: PathBuf) -> Result<(), anyhow::Error> {
        std::fs::copy(tmp.path(), &dst)?;
        Ok(())
    }

    #[test]
    fn insert_item() -> Result<(), anyhow::Error> {
        let (pool, _guard) = prepare_db()?;

        let repo = DieselItemRepository::new(pool)?;
        let stash: PublicStashData = serde_json::from_str(&PUBLIC_STASH_DATA)?;

        repo.insert_raw_item(&stash)?;

        let latest_stash_id = repo.get_stash_id()?;
        assert_eq!(
            latest_stash_id.latest_stash_id.unwrap(),
            "2949-5227-4536-5447-1849"
        );
        Ok(())
    }

    #[test]
    fn get_items() -> Result<(), anyhow::Error> {
        let (pool, _guard) = prepare_db()?;

        let repo = DieselItemRepository::new(pool)?;
        let stash: PublicStashData = serde_json::from_str(&PUBLIC_STASH_DATA)?;

        repo.insert_raw_item(&stash)?;
        let items = repo.get_items_by_basetype("Recurve Bow")?;

        for i in items {
            println!("{:?}", i);
        }

        Ok(())
    }

    #[test]
    fn set_latest_stash_id() -> Result<(), anyhow::Error> {
        let (pool, _guard) = prepare_db()?;
        let repo = DieselItemRepository::new(pool)?;

        repo.set_stash_id("dsa")?;
        repo.set_stash_id("asd-dsa")?;
        let stash = repo.get_stash_id()?;

        assert_eq!(stash.latest_stash_id, Some("asd-dsa".to_owned()));

        Ok(())
    }

    #[test]
    fn get_item_hybrids() -> Result<(), anyhow::Error> {
        let (pool, _guard) = prepare_db()?;

        let repo = DieselItemRepository::new(pool)?;
        let stash: PublicStashData = serde_json::from_str(&PUBLIC_STASH_DATA)?;

        repo.insert_raw_item(&stash)?;
        let items = repo.get_items_by_basetype("Vaal Haste")?;
        let item = items.first().unwrap();

        assert_eq!(item.hybrid.is_vaal_gem, true);
        assert_eq!(item.hybrid.base_type_name, "Haste");
        assert_eq!(item.hybrid.sec_descr_text, Some("Casts a temporary aura that increases the movement speed, attack speed and cast speed of you and your allies.".to_owned()));

        Ok(())
    }

    #[test]
    fn insert_remove_stash() -> Result<(), anyhow::Error> {
        let (pool, _guard) = prepare_db()?;

        let repo = DieselItemRepository::new(pool)?;
        let mut stash: PublicStashData = serde_json::from_str(&PUBLIC_STASH_DATA)?;

        repo.insert_raw_item(&stash.clone())?;

        stash.stashes = vec![stash
            .stashes
            .into_iter()
            .filter(|v| v.account_name.is_some())
            .nth(0)
            .unwrap()];
        stash.stashes.get_mut(0).unwrap().items.truncate(3);

        let _ = repo.insert_raw_item(&stash)?;

        let items = repo._get_raw_items(
            stash.stashes[0].account_name.as_ref().unwrap(),
            stash.stashes[0].stash.as_ref().unwrap(),
        )?;
        assert_eq!(items.len(), 3);
        Ok(())
    }
}
