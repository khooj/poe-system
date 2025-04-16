import { SingleItemWithConfig } from "./SingleItemWithConfig";
import { Collapse, Form } from "react-bootstrap";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { useContext, useId, useState } from "react";
import { ItemsContext } from "@states/preview";
import { useStore } from "zustand";
import { Button } from "react-bootstrap";

type MultipleItemsWithConfigProps = {
  itemKey: keyof BuildItemsWithConfig,
};

export const MultipleItemsWithConfig = ({ itemKey }: MultipleItemsWithConfigProps) => {
  const store = useContext(ItemsContext);
  if (!store) throw new Error('missing items context');
  const data = useStore(store, s => s.data);
  const item = data.provided[itemKey];

  if (!Array.isArray(item)) {
    throw new Error('passed single item to multiple items component');
  }

  const [open, setOpen] = useState(false);
  const id = useId();

  return <div>
    <Button onClick={() => setOpen(!open)} aria-controls={id} aria-expanded={open}>{itemKey}</Button>
    <Collapse in={open}>
      <div id={id}>
        {item.map((_, idx) => <SingleItemWithConfig itemKey={itemKey} multipleIndex={idx} />)}
      </div>
    </Collapse>
  </div>

};

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
