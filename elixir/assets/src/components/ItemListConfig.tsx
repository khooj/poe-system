import { ItemWithConfig } from "@bindings/domain/bindings/ItemWithConfig";
import { TypedItem } from "@bindings/domain/bindings/TypedItem";
import { ItemWithConfigComponent } from "./ItemWithConfig";

type ItemType = ItemWithConfig | ItemWithConfig[] | TypedItem | TypedItem[] | null;
type Props = {
  [item: string]: ItemType;
};

const isItemWithConfig = (v: ItemType): v is ItemWithConfig => {
  return !!v && (v as ItemWithConfig).config !== undefined;
};

const isItemWithConfigArray = (v: ItemType): v is ItemWithConfig[] => {
  // TODO: change config check in possible empty array
  return !!v && Array.isArray(v) && v.some(isItemWithConfig)
};

const isTypedItem = (v: ItemType): v is TypedItem => {
  return !!v && (v as TypedItem).id !== undefined;
};

const isTypedItemArray = (v: ItemType): v is TypedItem[] => {
  return !!v && Array.isArray(v) && v.some(isTypedItem)
};

export const ItemListConfig = (items: Props) => {
  const renderItem = (_k: string, v: ItemType) => {
    if (isItemWithConfig(v)) {
      return <ItemWithConfigComponent item={v} />
    }
    if (isItemWithConfigArray(v)) {
      return v.map(item => <ItemWithConfigComponent item={item} />);
    }
    if (isTypedItem(v) || isTypedItemArray(v)) {
      return <span>error rendering</span>
    }

    return null;
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
