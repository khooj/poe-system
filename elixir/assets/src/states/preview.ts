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
}

interface ItemsState extends PreviewProps {
  setItemConfig: (k: keyof BuildItemsWithConfig, stat_id: string, cfg: ModConfig | null) => void,
}

export type ItemsStore = ReturnType<typeof createItemsStore>;

export const createItemsStore = (initProps: PreviewProps) => {
  return createStore<ItemsState>()(devtools(immer((set) => ({
    ...initProps,
    setItemConfig: (k: keyof BuildItemsWithConfig, stat_id: string, cfg: ModConfig | null) => set((state) => {
      const item = state.data.provided[k];
      if (Array.isArray(item)) {
        return;
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

