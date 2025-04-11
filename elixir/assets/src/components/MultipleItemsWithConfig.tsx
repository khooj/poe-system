import { Button } from "react-bootstrap";
import { useContext, useId, useState } from "react";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { ItemsContext } from "@states/preview";
import { useStore } from "zustand";
import { Collapse } from "react-bootstrap";
import { SingleItemWithConfig } from "./SingleItemWithConfig";

type Props = {
  itemKey: keyof BuildItemsWithConfig,
};

export const MultipleItemsWithConfig = ({ itemKey }: Props) => {
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
