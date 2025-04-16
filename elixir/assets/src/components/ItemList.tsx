import { ItemWithConfig } from "@bindings/domain/bindings/ItemWithConfig";
import RequiredItemComponent from "./RequiredItemComponent";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { useId, useState } from "react";
import { Button, Collapse } from "react-bootstrap";
import { ModConfig } from "@bindings/domain/bindings/ModConfig";

type RenderConfigProps = {
  cf: ModConfig | null,
};

const RenderConfig = ({ cf }: RenderConfigProps) => {
  if (!cf) {
    return <></>
  }

  if (cf === 'Exist') {
    return <span>(exist)</span>
  } else if ('Exact' in cf) {
    return <span>(exact: {cf.Exact})</span>
  } else if ('Range' in cf) {
    return <span>(range: {cf.Range.start}-{cf.Range.end})</span>
  } else if ('Min' in cf) {
    return <span>(min: {cf.Min})</span>
  } else if ('Max' in cf) {
    return <span>(max: {cf.Max})</span>
  }
};

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
        {items.map(si => <RequiredItemComponent
          item={si.item}
          modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
        />)}
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
      return <RequiredItemComponent
        item={v.item}
        modConfigComponent={(mcf) => <RenderConfig cf={mcf[1]} />}
      />
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
