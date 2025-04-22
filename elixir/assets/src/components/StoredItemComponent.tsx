import { Price } from "@bindings/domain/bindings/Price";
import { StoredItem } from "@bindings/domain/bindings/StoredItem";
import { StoredItemInfo } from "@bindings/domain/bindings/StoredItemInfo";

const isNotGem = (v: StoredItemInfo) => {
  return v.type !== "Gem";
}

const RenderCost = ({ price }: { price: Price }) => {
  if ('Chaos' in price) {
    return <div>cost: {price.Chaos} chaos</div>
  }
  if ('Divine' in price) {
    return <div>cost: {price.Divine} divine</div>
  }

  return <div>cost: {price.Custom[1]} {price.Custom[0]}</div>
};

type StoredItemProps = {
  item: StoredItem,
};

export const StoredItemComponent = ({ item }: StoredItemProps) => {
  if (isNotGem(item.info)) {
    return <div className='border border-primary m-2 flex-fill' style={{ fontSize: '14px' }}>
      <div className='border d-flex justify-content-between'>
        <span>{item.name}<br />{item.basetype}</span>
        <RenderCost price={item.price} />
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
