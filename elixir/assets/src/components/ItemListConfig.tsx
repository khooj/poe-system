import { ItemWithConfig } from "@bindings/domain/bindings/ItemWithConfig";
import { ItemWithConfigComponent } from "./ItemWithConfig";
import { Form } from "react-bootstrap";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { useContext } from "react";
import { ItemsContext } from "@states/preview";
import { useStore } from "zustand";

type ItemType = ItemWithConfig | ItemWithConfig[];

const isItemWithConfig = (v: ItemType): v is ItemWithConfig => {
  return !!v && !Array.isArray(v);
};

const isItemWithConfigArray = (v: ItemType): v is ItemWithConfig[] => {
  return !!v && Array.isArray(v);
};

export const ItemListConfig = () => {
  const store = useContext(ItemsContext);
  if (!store) throw new Error('missing items context');

  const data = useStore(store, s => s.data);
  const renderItem = (k: keyof BuildItemsWithConfig, v: ItemWithConfig | ItemWithConfig[]) => {
    if (isItemWithConfig(v)) {
      return <ItemWithConfigComponent itemKey={k} />
    }
    if (isItemWithConfigArray(v)) {
      return <ItemWithConfigComponent itemKey={k} />
    }

    throw new Error("unknown item type");
  };

  const renderList = () => {
    if (!Object.entries(data.provided).some(i => !!i[1])) {
      return <div>Nothing found yet</div>
    }

    return Object.entries(data.provided).map(([k, v]) => {
      return renderItem(k as keyof BuildItemsWithConfig, v);
    }).flat();
  };

  return <div className="d-flex flex-column">
    <Form encType="multipart/form-data">
      {renderList()}
    </Form>
  </div>
};
