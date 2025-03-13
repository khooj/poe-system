import { ItemWithConfig } from "@bindings/domain/bindings/ItemWithConfig";
import { TypedItem } from "@bindings/domain/bindings/TypedItem";
import { ItemWithConfigComponent } from "./ItemWithConfig";
import TypedItemComponent from "./TypedItemComponent";

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

export const ItemList = (items: Props) => {
  const renderItem = (k: string, v: ItemType) => {
    if (isItemWithConfig(v)) {
      return <ItemWithConfigComponent item={v} />
    }
    if (isItemWithConfigArray(v)) {
      return <div className="d-flex flex-row">
        {v.map(item => <ItemWithConfigComponent item={item} />)}
      </div>
    }
    if (isTypedItem(v)) {
      return <TypedItemComponent item={v} />
    }
    if (isTypedItemArray(v)) {
      return <div className="d-flex flex-row">
        {v.map(item => <TypedItemComponent item={item} />)}
      </div>
    }

    return null;
  };

  return <div className="d-flex flex-column">
    {
      Object.entries(items).map(([k, v]) => {
        return renderItem(k, v);
      })
    }
  </div>
};
