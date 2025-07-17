import { StoredItemInfo } from "@bindings/domain/bindings/StoredItemInfo";

export const isNotGem = (v: StoredItemInfo) => {
  return v.type !== "Gem";
}
