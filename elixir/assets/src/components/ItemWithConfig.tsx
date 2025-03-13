import { ItemWithConfig } from "@bindings/domain/bindings/ItemWithConfig";
import TypedItemComponent from "./TypedItemComponent";

type Props = {
  item: ItemWithConfig
};

export const ItemWithConfigComponent = ({ item }: Props) => {
  return <div>
    <TypedItemComponent item={item.item} />
  </div>
};
