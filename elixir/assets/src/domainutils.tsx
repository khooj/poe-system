import { ItemInfo } from "@bindings/domain/bindings/ItemInfo";
import { StoredItemInfo } from "@bindings/domain/bindings/StoredItemInfo";

export const isNotGem = (v: ItemInfo | StoredItemInfo) => {
  return v.type !== "Gem";
}
