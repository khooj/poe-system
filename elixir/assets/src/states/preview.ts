import { isNotGem } from "@/domainutils";
import { BuildInfo } from "@bindings/domain/bindings/BuildInfo";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { ItemWithConfig } from "@bindings/domain/bindings/ItemWithConfig";
import { create, createStore } from "zustand";
import { persist, devtools } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";
import { createContext } from "react";
import { ModConfig } from "@bindings/domain/bindings/ModConfig";
import Routes from "@routes";
import { InertiaFormProps } from "@inertiajs/react";

export type InertiaFormType = {
  config: BuildInfo,
  id: string
};

interface PreviewProps {
  data: BuildInfo,
}

interface ItemsState extends PreviewProps {
  setItemConfig: (k: keyof BuildItemsWithConfig, stat_id: string, cfg: ModConfig) => void,
}

export type ItemsStore = ReturnType<typeof createItemsStore>;

export const createItemsStore = (initProps: PreviewProps) => {
  return createStore<ItemsState>()(devtools(immer((set) => ({
    ...initProps,
    setItemConfig: (k: keyof BuildItemsWithConfig, stat_id: string, cfg: ModConfig) => set((state) => {
      console.log('called set item config with args', k, stat_id, cfg);
      const item = state.data.provided[k];
      if (Array.isArray(item)) {
        return;
      }
      if (!isNotGem(item.item.info)) {
        return;
      }

      const itemModIdx = item.item.info.mods.findIndex(([m, _]) => m.stat_id === stat_id);
      state.data.provided[k].item.info.mods[itemModIdx][1] = cfg;
      // state.data.provided[k].item.info.mods[itemModIdx].pop();
      // state.data.provided[k].item.info.mods[itemModIdx].push(cfg);
    }),
  }))));
};

export const ItemsContext = createContext<ItemsStore | null>(null);

