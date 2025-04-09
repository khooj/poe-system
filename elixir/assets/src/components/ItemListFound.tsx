import { StoredItem } from "@bindings/domain/bindings/StoredItem";
import { StoredItemInfo } from "@bindings/domain/bindings/StoredItemInfo";
import { StoredMod } from "@bindings/domain/bindings/StoredMod";

type ItemType = StoredItem | StoredItem[] | null;
type Props = {
  [item: string]: ItemType;
};

const isSingleItem = (v: ItemType): v is StoredItem => {
  return !!v && !Array.isArray(v);
};

const isNotGem = (v: StoredItemInfo) => {
  return v.type !== "Gem";
}

export const ItemListFound = (items: Props) => {
  const renderItem = (k: string, item: ItemType) => {
    if (isSingleItem(item)) {
      const renderMods = (mods: StoredMod[]) => {
        return mods.map((m) => <div className=''>{m.text}</div>)
      };

      if (isNotGem(item.info)) {
        return <div className='border border-primary m-2 flex-fill' style={{ fontSize: '14px' }}>
          <div className='border'>
            <span>{item.name}<br />{item.basetype}</span>
          </div>
          <div className='border'>
            <div>{renderMods(item.info.mods)}</div>
          </div>
        </div >;
      } else {
        return <div className='m-2 border' style={{ fontSize: '14px' }}>
          <p>{item.name} {item.info.level}lvl/+{item.info.quality}%</p>
        </div>;
      }
    }
    // // if (isItemWithConfigArray(v)) {
    // //   return v.map(item => <ItemWithConfigComponent item={item} />);
    // // }
    // if (isRequiredItem(v)) {
    //   return <RequiredItemComponent item={v} />
    // }
    // // if (isRequiredItemArray(v)) {
    // //   return v.map(item => <RequiredItemComponent item={item} />);
    // // }

    return <p>not rendering for now</p>;
  };

  const renderList = (items: Props) => {
    if (!Object.entries(items).some(i => !!i[1])) {
      return <div>Nothing found yet</div>
    }

    return Object.entries(items).map(([k, v]) => {
      return renderItem(k, v);
    }).flat();
  };

  return <div className="d-flex flex-column">
    {renderList(items)}
  </div>
};
