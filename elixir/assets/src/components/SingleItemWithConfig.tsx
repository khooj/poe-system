import { ModConfig } from "@bindings/domain/bindings/ModConfig";
import { useCallback, useContext } from "react";
import { Mod } from "@bindings/domain/bindings/Mod";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { ItemsContext } from "@states/preview";
import { useStore } from "zustand";
import Item from './Item';
import { Flex, Grid, Group, RangeSlider, Select } from "@mantine/core";
import { useDebouncedCallback } from "@mantine/hooks";
import { ItemMod } from "./ItemMod";

type Props = {
  itemKey: keyof BuildItemsWithConfig,
  multipleIndex?: number,
};

export const SingleItemWithConfig = ({ itemKey, multipleIndex }: Props) => {
  const store = useContext(ItemsContext);
  if (!store) throw new Error('missing items context');
  const data = useStore(store, s => s.data);
  const item = data.provided[itemKey];

  const item2 = Array.isArray(item) ? item[multipleIndex!] : item;

  return <Item
    item={item2.item}
    modConfigComponent={([m, cf], idx) => <div>
      <ItemMod key={idx} k={itemKey} m={m} origCfg={cf} multipleIndex={multipleIndex} />
    </div>}
  />
};
