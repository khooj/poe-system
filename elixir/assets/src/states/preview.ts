import { isNotGem } from "@/domainutils";
import { BuildInfo } from "@bindings/domain/bindings/BuildInfo";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { createStore } from "zustand";
import { devtools } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";
import { createContext } from "react";
import { ModConfig } from "@bindings/domain/bindings/ModConfig";


interface PreviewProps {
  data: BuildInfo,
  enabled: boolean,
}

interface ItemsState extends PreviewProps {
  disableEdit: () => void,
  enableEdit: () => void,
  setItemConfig: (k: keyof BuildItemsWithConfig, stat_id: string, cfg: ModConfig | null, multipleIndex?: number) => void,
}

export type ItemsStore = ReturnType<typeof createItemsStore>;

export const createItemsStore = (initProps: PreviewProps) => {
  return createStore<ItemsState>()(devtools(immer((set) => ({
    ...initProps,
    disableEdit: () => set((state) => {
      state.enabled = false;
    }),
    enableEdit: () => set((state) => {
      state.enabled = true;
    }),
    setItemConfig: (k: keyof BuildItemsWithConfig, stat_id: string, cfg: ModConfig | null, multipleIndex?: number) => set((state) => {
      const item = state.data.provided[k];
      if (Array.isArray(item)) {
        const itemIndexed = item[multipleIndex!].item;
        if (!isNotGem(itemIndexed.info)) {
          return
        }
        const itemModIdx = itemIndexed.info.mods.findIndex(mcf => mcf[0].stat_id === stat_id);
        itemIndexed.info.mods[itemModIdx][1] = cfg;
      } else {
        if (!isNotGem(item.item.info)) {
          return;
        }
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        const itemModIdx = item.item.info.mods.findIndex(([m, _]) => m.stat_id === stat_id);
        // @ts-expect-error ts typecheck error
        state.data.provided[k].item.info.mods[itemModIdx][1] = cfg;
      }
    }),
  }))));
};

export const ItemsContext = createContext<ItemsStore | null>(null);

