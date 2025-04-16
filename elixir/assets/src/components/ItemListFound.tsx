import { FoundBuildItems } from "@bindings/domain/bindings/FoundBuildItems";
import { StoredItem } from "@bindings/domain/bindings/StoredItem";
import { useId, useState } from "react";
import { Collapse } from "react-bootstrap";
import { Button } from "react-bootstrap";
import { StoredItemComponent } from "./StoredItemComponent";

const isSingleItem = (v: ItemType): v is StoredItem => {
  return !!v && !Array.isArray(v);
};

type MultipleItemsProps = {
  items: StoredItem[],
  itemKey: string,
};

const MultipleItems = ({ items, itemKey }: MultipleItemsProps) => {
  const [open, setOpen] = useState(false);
  const id = useId();

  return <div>
    <Button onClick={() => setOpen(!open)} aria-controls={id} aria-expanded={open}>{itemKey}</Button>
    <Collapse in={open}>
      <div id={id}>
        {items.map(si => <StoredItemComponent item={si} />)}
      </div>
    </Collapse>
  </div>
};

const itemsOrder: (keyof FoundBuildItems)[] = [
  'helmet', 'body', 'gloves', 'boots',
  'weapon1', 'weapon2', 'belt', 'amulet',
  'ring1', 'ring2', 'jewels', 'gems',
  'flasks'
];

type ItemType = StoredItem | StoredItem[] | null;
type Props = {
  [item: string]: ItemType;
};

export const ItemListFound = (items: Props) => {
  const renderItem = (k: string, item: ItemType) => {
    if (!item) {
      return <div>Item ({k}) not found</div>
    } else if (isSingleItem(item)) {
      return <StoredItemComponent item={item} />
    } else {
      return <MultipleItems items={item} itemKey={k} />
    }
  };

  const renderList = (items: Props) => {
    if (!Object.entries(items).some(i => !!i[1])) {
      return <div>Nothing found yet</div>
    }

    return itemsOrder.map(k => {
      return renderItem(k, items[k]);
    })
  };

  return <div className="d-flex flex-column">
    {renderList(items)}
  </div>
};
