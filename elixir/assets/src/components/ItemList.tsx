import { ItemWithConfig } from "@bindings/domain/bindings/ItemWithConfig";
import RequiredItemComponent from "./RequiredItemComponent";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { useId, useState } from "react";
import { Button, Collapse } from "react-bootstrap";

type MultipleRequiredProps = {
  items: ItemWithConfig[],
  itemKey: string,
};

const MultipleRequired = ({ items, itemKey }: MultipleRequiredProps) => {
  const [open, setOpen] = useState(false);
  const id = useId();

  return <div>
    <Button onClick={() => setOpen(!open)} aria-controls={id} aria-expanded={open}>{itemKey}</Button>
    <Collapse in={open}>
      <div id={id}>
        {items.map(si => <RequiredItemComponent item={si.item} />)}
      </div>
    </Collapse>
  </div>
};

type ItemType = ItemWithConfig | ItemWithConfig[];
type Props = {
  [item: string]: ItemType;
};

const isItemWithConfig = (v: ItemType): v is ItemWithConfig => {
  return !!v && (v as ItemWithConfig).item !== undefined;
};

const itemsOrder: (keyof BuildItemsWithConfig)[] = [
  'helmet', 'body', 'gloves', 'boots',
  'weapon1', 'weapon2', 'belt', 'amulet',
  'ring1', 'ring2', 'jewels', 'gems',
  'flasks'
];

export const ItemList = (items: Props) => {
  const renderItem = (k: string, v: ItemType) => {
    if (isItemWithConfig(v)) {
      return <RequiredItemComponent item={v.item} />
    } else {
      return <MultipleRequired items={v} itemKey={k} />
    }
  };

  const renderList = (items: Props) => {
    if (!Object.entries(items).some(i => !!i[1])) {
      return <div>Nothing found yet</div>
    }

    return itemsOrder.map((k) => {
      return renderItem(k, items[k]);
    });
  };

  return <div className="d-flex flex-column">
    {renderList(items)}
  </div>
};
