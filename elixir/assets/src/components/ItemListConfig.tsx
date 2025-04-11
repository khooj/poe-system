import { SingleItemWithConfig } from "./SingleItemWithConfig";
import { Form } from "react-bootstrap";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { useContext } from "react";
import { ItemsContext } from "@states/preview";
import { useStore } from "zustand";
import { MultipleItemsWithConfig } from "./MultipleItemsWithConfig";

const itemsOrder: (keyof BuildItemsWithConfig)[] = [
  'helmet', 'body', 'gloves', 'boots',
  'weapon1', 'weapon2', 'belt', 'amulet',
  'ring1', 'ring2', 'jewels', 'gems',
  'flasks'
];

export const ItemListConfig = () => {
  const store = useContext(ItemsContext);
  if (!store) throw new Error('missing items context');
  const data = useStore(store, s => s.data);

  return <div className="d-flex flex-column">
    <Form encType="multipart/form-data">
      {itemsOrder.map(k => {
        if (!Array.isArray(data.provided[k])) {
          return <SingleItemWithConfig itemKey={k} />
        } else {
          return <MultipleItemsWithConfig itemKey={k} />
        }
      })}
    </Form>
  </div>
};
