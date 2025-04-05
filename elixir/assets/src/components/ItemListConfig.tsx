import { ItemWithConfig } from "@bindings/domain/bindings/ItemWithConfig";
import { ItemWithConfigComponent } from "./ItemWithConfig";
import { Form } from "react-bootstrap";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { BuildInfo } from "@bindings/domain/bindings/BuildInfo";

type ItemType = ItemWithConfig | ItemWithConfig[];

const isItemWithConfig = (v: ItemType): v is ItemWithConfig => {
  return !!v && !Array.isArray(v);
};

const isItemWithConfigArray = (v: ItemType): v is ItemWithConfig[] => {
  return !!v && Array.isArray(v);
};

type Props = {
  data: BuildInfo,
  setItemCb: (k: keyof BuildItemsWithConfig, it: ItemWithConfig | ItemWithConfig[]) => void,
};

export const ItemListConfig = ({ data, setItemCb }: Props) => {
  const renderItem = (k: keyof BuildItemsWithConfig, v: ItemWithConfig | ItemWithConfig[]) => {
    if (isItemWithConfig(v)) {
      return <ItemWithConfigComponent itemKey={k} item={v} setItemCb={setItemCb} />
    }
    if (isItemWithConfigArray(v)) {
      return <ItemWithConfigComponent itemKey={k} items={v} setItemCb={setItemCb} />
    }

    throw new Error("unknown item type");
  };

  const renderList = () => {
    if (!Object.entries(data.provided).some(i => !!i[1])) {
      return <div>Nothing found yet</div>
    }

    return Object.entries(data.provided).map(([k, v]) => {
      return renderItem(k as keyof BuildItemsWithConfig, v);
    }).flat();
  };

  return <div className="d-flex flex-column">
    <Form encType="multipart/form-data">
      {renderList()}
    </Form>
  </div>
};
