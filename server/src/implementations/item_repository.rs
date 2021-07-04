use super::models::{
    Extended, Hybrid, HybridModDb, IncubatedItem, Influence, Mod, NewHybrid, NewHybridMod, NewItem,
    NewLatestStash, Property, RawItem, RemoveItems, Socket, SocketedItem, SplittedItem,
    Subcategory, UltimatumMod,
};
use super::TypedConnectionPool;
use crate::domain::item::Item as DomainItem;
use crate::ports::outbound::public_stash_retriever::PublicStashData;
use crate::ports::outbound::repository::{LatestStashId, RepositoryError};
use diesel::prelude::*;
use diesel::BelongingToDsl;
use itertools::Itertools;
use std::{
    collections::HashMap,
    convert::{From, TryFrom},
};
use tracing::warn;
use uuid::Uuid;

macro_rules! collect_val {
    ($v:expr, $field:tt) => {
        $v.iter()
            .map(|el| &el.$field)
            .filter_map(|el| el.as_ref())
            .flatten()
    };
}

macro_rules! collect_val2 {
    ($v:expr, $field:tt) => {
        $v.iter().map(|el| &el.$field).filter_map(|el| el.as_ref())
    };
}

#[derive(Clone)]
pub struct DieselItemRepository {
    conn: TypedConnectionPool,
}

impl DieselItemRepository {
    pub fn new(conn: TypedConnectionPool) -> Result<DieselItemRepository, RepositoryError> {
        Ok(DieselItemRepository { conn })
    }

    pub fn get_items_by_basetype(
        &self,
        base_type_: &str,
    ) -> Result<Vec<DomainItem>, RepositoryError> {
        use crate::schema::hybrid_mods::dsl as hybrid_mods_dsl;
        use crate::schema::items::dsl::{base_type, items as items_table};
        use itertools::izip;

        let conn = self.conn.get()?;

        let items = items_table
            .filter(base_type.eq(base_type_))
            .load::<RawItem>(&conn)?;

        let influences = Influence::belonging_to(&items)
            .load::<Influence>(&conn)?
            .grouped_by(&items);
        let mods = Mod::belonging_to(&items)
            .load::<Mod>(&conn)?
            .grouped_by(&items);
        let extended = Extended::belonging_to(&items)
            .load::<Extended>(&conn)?
            .grouped_by(&items);

        let hybrid = Hybrid::belonging_to(&items).load::<Hybrid>(&conn)?;
        let hybrid_mods = hybrid_mods_dsl::hybrid_mods
            .filter(hybrid_mods_dsl::id.eq_any(hybrid.iter().map(|e| &e.hybrid_id)))
            .load::<HybridModDb>(&conn)?;

        let hybrid_mods = hybrid_mods
            .into_iter()
            .map(|el| (el.id.clone(), el))
            .collect::<HashMap<String, HybridModDb>>();

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
        let properties = Property::belonging_to(&items)
            .load::<Property>(&conn)?
            .grouped_by(&items);
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

    fn _get_raw_items(
        &self,
        account_name_: &str,
        stash_id_: &str,
    ) -> Result<Vec<RawItem>, RepositoryError> {
        use crate::schema::items::dsl::*;

        let conn = self.conn.get()?;

        Ok(items
            .filter(account_name.eq(account_name_).and(stash_id.eq(stash_id_)))
            .load::<RawItem>(&conn)?)
    }

    pub fn get_stash_id(&self) -> Result<LatestStashId, RepositoryError> {
        use crate::schema::latest_stash_id::dsl::*;

        let conn = self.conn.get()?;

        let v = latest_stash_id.load::<LatestStashId>(&conn)?;
        Ok(v.into_iter()
            .nth(0)
            .or(Some(LatestStashId::default()))
            .unwrap())
    }

    pub fn set_stash_id(&self, id_: &str) -> Result<(), RepositoryError> {
        use crate::schema::latest_stash_id::dsl::*;

        let conn = self.conn.get()?;

        // workaround for upsert functionality for sqlite https://github.com/diesel-rs/diesel/issues/1854
        // TODO: use replace_into instead
        let vals = latest_stash_id.load::<LatestStashId>(&conn)?;
        if vals.len() == 0 {
            let latest_stash = NewLatestStash { id: id_.to_owned() };
            diesel::insert_into(latest_stash_id)
                .values(&latest_stash)
                .execute(&conn)?;
        } else {
            diesel::update(latest_stash_id)
                .set(id.eq(id_))
                .execute(&conn)?;
        }
        Ok(())
    }

    fn save_new_hybrid_mod(&self, mod_: NewHybridMod) -> Result<String, RepositoryError> {
        use crate::schema::hybrid_mods::dsl::*;

        let conn = self.conn.get()?;

        match diesel::insert_into(hybrid_mods)
            .values(&mod_)
            .execute(&conn)
        {
            Ok(_) => Ok(mod_.id),
            Err(_) => {
                let q = hybrid_mods
                    .filter(
                        is_vaal_gem
                            .eq(&mod_.is_vaal_gem)
                            .and(base_type_name.eq(&mod_.base_type_name)),
                    )
                    .select(id);

                // TODO: use tracing for proper debugging
                // use diesel::sqlite::Sqlite;
                // println!("{}", debug_query::<Sqlite, _>(&q));
                Ok(q.first::<String>(&conn)?)
            }
        }
    }

    pub fn insert_raw_item(&self, public_data: &PublicStashData) -> Result<(), RepositoryError> {
        use crate::schema::items::dsl::*;
        use crate::schema::{
            extended::dsl::extended as extended_table, hybrids::dsl::hybrids as hybrids_table,
            incubated_item::dsl::incubated_item as incubated_item_table,
            influences::dsl::influences as influences_table, mods::dsl::mods as mods_table,
            properties::dsl::properties as properties_table,
            sockets::dsl::sockets as sockets_table,
            subcategories::dsl::subcategories as subcategories_table,
            ultimatum_mods::dsl::ultimatum_mods as ultimatum_mods_table,
        };
        use itertools::izip;

        let conn = self.conn.get()?;

        conn.transaction::<_, RepositoryError, _>(|| {
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
                                warn!("skipping {:?} item because cant generate entity", i);
                                None
                            }
                        })
                        .filter_map(|i| i)
                        .collect::<Vec<_>>()
                })
                .flatten()
                .into_group_map_by(|el| el.item.account_id.clone());

            // TODO: check if works in given transaction
            self.set_stash_id(&public_data.next_change_id)?;

            for (k, v) in &new_item_info {
                if v.len() == 0 {
                    diesel::delete(items.filter(account_id.eq(&k))).execute(&conn)?;
                } else {
                    let insert_items: Vec<&NewItem> = v.iter().map(|v| &v.item).collect();
                    let delete_items: Vec<RemoveItems> = v
                        .iter()
                        .map(|v| RemoveItems {
                            account_name: &v.item.account_name,
                            stash_id: &v.item.stash_id,
                        })
                        .collect();

                    // TODO: somehow use Identifiable or smth else to simplify delete query
                    for i in &delete_items {
                        diesel::delete(
                            items.filter(
                                account_name.eq(i.account_name).and(stash_id.eq(i.stash_id)),
                            ),
                        )
                        .execute(&conn)?;
                    }

                    for i in insert_items {
                        diesel::insert_into(items).values(i).execute(&conn)?;
                    }

                    for mods in izip!(
                        collect_val!(v, mods),
                        collect_val!(v, subcategories),
                        collect_val!(v, props),
                        collect_val!(v, sockets),
                        collect_val!(v, ultimatum),
                    ) {
                        diesel::insert_into(mods_table)
                            .values(mods.0)
                            .execute(&conn)?;

                        diesel::insert_into(subcategories_table)
                            .values(mods.1)
                            .execute(&conn)?;

                        diesel::insert_into(properties_table)
                            .values(mods.2)
                            .execute(&conn)?;

                        diesel::insert_into(sockets_table)
                            .values(mods.3)
                            .execute(&conn)?;

                        diesel::insert_into(ultimatum_mods_table)
                            .values(mods.4)
                            .execute(&conn)?;
                    }

                    for mods in izip!(
                        collect_val2!(v, incubated),
                        collect_val2!(v, hybrid),
                        collect_val2!(v, extended),
                        collect_val2!(v, influence),
                    ) {
                        diesel::insert_into(incubated_item_table)
                            .values(mods.0)
                            .execute(&conn)?;

                        let id_mod = self.save_new_hybrid_mod(NewHybridMod {
                            id: Uuid::new_v4().to_hyphenated().to_string(),
                            is_vaal_gem: mods.1.is_vaal_gem,
                            base_type_name: mods.1.base_type_name.clone(),
                            sec_descr_text: mods.1.sec_descr_text.clone(),
                        })?;

                        diesel::insert_into(hybrids_table)
                            .values(&NewHybrid {
                                hybrid_id: id_mod.clone(),
                                item_id: mods.1.item_id.clone(),
                            })
                            .execute(&conn)?;

                        diesel::insert_into(extended_table)
                            .values(mods.2)
                            .execute(&conn)?;

                        diesel::insert_into(influences_table)
                            .values(mods.3)
                            .execute(&conn)?;
                    }
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
    use crate::{
        implementations::models::NewHybridMod,
        ports::outbound::public_stash_retriever::PublicStashData,
    };
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::sqlite::SqliteConnection;
    use std::path::PathBuf;
    use temp_file::{empty, TempFile};
    use uuid::Uuid;

    const PUBLIC_STASH_DATA: &str = include_str!("public-stash-tabs.json");

    embed_migrations!("migrations");

    fn prepare_db() -> Result<(Pool<ConnectionManager<SqliteConnection>>, TempFile), anyhow::Error>
    {
        let f = empty();

        let pool = Pool::new(ConnectionManager::new(f.path().to_str().unwrap()))?;

        {
            let conn = pool.get()?;
            embedded_migrations::run(&conn)?;
        }

        Ok((pool, f))
    }

    fn _copy_file(tmp: &TempFile, dst: PathBuf) -> Result<(), anyhow::Error> {
        // let src = File::open(tmp.path())?;
        // let dst = OpenOptions::new().create(true).open(&dst)?;
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
    fn save_hybrid() -> Result<(), anyhow::Error> {
        let (pool, _guard) = prepare_db()?;

        let repo = DieselItemRepository::new(pool)?;

        let id_1 = repo.save_new_hybrid_mod(NewHybridMod {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            is_vaal_gem: true,
            base_type_name: "Haste".to_owned(),
            sec_descr_text: Some("test".to_owned()),
        })?;
        let id_2 = repo.save_new_hybrid_mod(NewHybridMod {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            is_vaal_gem: true,
            base_type_name: "Haste".to_owned(),
            sec_descr_text: Some("test".to_owned()),
        })?;

        assert_eq!(id_1, id_2);

        let id1 = repo.save_new_hybrid_mod(NewHybridMod {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            is_vaal_gem: true,
            base_type_name: "Haste".to_owned(),
            sec_descr_text: None,
        })?;
        let id2 = repo.save_new_hybrid_mod(NewHybridMod {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            is_vaal_gem: true,
            base_type_name: "Haste".to_owned(),
            sec_descr_text: None,
        })?;

        assert_eq!(id1, id2);

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
