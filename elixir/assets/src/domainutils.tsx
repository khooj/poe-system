import { ItemInfo } from "@bindings/domain/bindings/ItemInfo";

export const isNotGem = (v: ItemInfo) => {
  return v.type !== "Gem";
}
