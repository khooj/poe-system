import { SingleItemWithConfig } from "./SingleItemWithConfig";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { useContext } from "react";
import { ItemsContext } from "@states/preview";
import { useStore } from "zustand";
import MultipleItems from "./MultipleItems";
import { Flex } from "@mantine/core";

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

  return <Flex direction='column'>
    <form encType="multipart/form-data">
      {itemsOrder.map(k => {
        if (!Array.isArray(data.provided[k])) {
          return <SingleItemWithConfig itemKey={k} />
        } else {
          return <MultipleItems itemKey={k}>
            {data.provided[k].map((_, idx) => <SingleItemWithConfig itemKey={k} multipleIndex={idx} />)}
          </MultipleItems>
        }
      })}
    </form>
  </Flex>
};
