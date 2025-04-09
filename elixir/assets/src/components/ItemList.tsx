import { ItemWithConfig } from "@bindings/domain/bindings/ItemWithConfig";
import RequiredItemComponent from "./RequiredItemComponent";

type ItemType = ItemWithConfig | ItemWithConfig[] | null;
type Props = {
  [item: string]: ItemType;
};

const isItemWithConfig = (v: ItemType): v is ItemWithConfig => {
  return !!v && (v as ItemWithConfig).item !== undefined;
};

export const ItemList = (items: Props) => {
  const renderItem = (k: string, v: ItemType) => {
    if (isItemWithConfig(v)) {
      return <RequiredItemComponent item={v.item} />
    }

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
