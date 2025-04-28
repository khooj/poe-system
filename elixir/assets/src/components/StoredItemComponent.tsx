import { Price } from "@bindings/domain/bindings/Price";
import { StoredItem } from "@bindings/domain/bindings/StoredItem";
import Item from "./Item";

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
  return <Item item={item} itemNameComponent={(i) => <RenderCost price={(i as StoredItem).price} />} />
};
