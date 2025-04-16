import { StoredItem } from "@bindings/domain/bindings/StoredItem";
import { StoredItemInfo } from "@bindings/domain/bindings/StoredItemInfo";

const isNotGem = (v: StoredItemInfo) => {
  return v.type !== "Gem";
}

type StoredItemProps = {
  item: StoredItem,
};

export const StoredItemComponent = ({ item }: StoredItemProps) => {
  if (isNotGem(item.info)) {
    return <div className='border border-primary m-2 flex-fill' style={{ fontSize: '14px' }}>
      <div className='border'>
        <span>{item.name}<br />{item.basetype}</span>
      </div>
      <div className='border'>
        <div>{item.info.mods.map(m => <div>{m.text}</div>)}</div>
      </div>
    </div >;
  } else {
    return <div className='m-2 border' style={{ fontSize: '14px' }}>
      <p>{item.name} {item.info.level}lvl/+{item.info.quality}%</p>
    </div>;
  }
};
