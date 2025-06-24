import { ModConfig } from "@bindings/domain/bindings/ModConfig";
import { useCallback, useContext } from "react";
import { Mod } from "@bindings/domain/bindings/Mod";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import { ItemsContext } from "@states/preview";
import { useStore } from "zustand";
import { ItemWithConfig } from "@bindings/domain/bindings/ItemWithConfig";
import RequiredItemComponent from "./RequiredItemComponent";
import { Flex, Grid, Group, RangeSlider, Select } from "@mantine/core";
import { useDebouncedCallback } from "@mantine/hooks";

const AddRangeSlider = ({ setCfgCb, enabled, origCfg }) => {
  const debouncedOnChange = (e: [number, number]) => {
    const start = e[0];
    const end = e[1];
    setCfgCb({ Range: { start, end } });
  };
  const { start, end } = origCfg.Range;

  return <RangeSlider
    miw="xs"
    maw={600}
    min={0}
    max={500}
    defaultValue={[start, end]}
    onChangeEnd={debouncedOnChange}
    disabled={!enabled}
    marks={[
      { value: 0, label: '0' },
      { value: 500, label: '500' },
    ]}
  />

};

type ItemWithConfigProps = {
  k: keyof BuildItemsWithConfig,
  m: Mod,
  origCfg: ModConfig | null,
  multipleIndex?: number,
};

const ItemModWithConfig = ({ k, m, origCfg, multipleIndex }: ItemWithConfigProps) => {
  const store = useContext(ItemsContext);
  if (!store) {
    throw new Error('missing context');
  }
  const setItemConfig = useStore(store, s => s.setItemConfig);
  const enabled = useStore(store, s => s.enabled);

  const defaultConfigValue = (config: ModConfig | null) => {
    if (!config) {
      return 'ignore';
    }
    if (config === "Exist") {
      return "Exist";
    }
    if ("Exact" in config) {
      return "Exact";
    }
    if ("Range" in config) {
      return "Range";
    }
    if ("Min" in config) {
      return "Min";
    }
    if ("Max" in config) {
      return "Max";
    }
    throw new Error("unknown config type");
  };

  const setCfgCb = useCallback((cf: ModConfig | null) => {
    setItemConfig(k, m.stat_id, cf, multipleIndex);
  }, [k, m, setItemConfig, multipleIndex]);

  const renderAdditionalForSelect = () => {
    if (!origCfg) {
      return <></>
    }

    if (origCfg === 'Exist') {
      return <></>
    }

    if ("Range" in origCfg) {
      return <AddRangeSlider origCfg={origCfg} setCfgCb={setCfgCb} enabled={enabled} />
    } else if ("Exact" in origCfg) {
      return <></>
    } else {
      return <p>not supported</p>
    }
  };

  const onChange = (e: string | null) => {
    if (e === 'Exist') {
      setCfgCb('Exist');
    } else if (e === 'Exact') {
      setCfgCb({ Exact: (m.current_value_int && m.current_value_int[0]) || (m.current_value_float && m.current_value_float[0]) || 0 });
    } else if (e === 'Range') {
      setCfgCb({ Range: { start: 0, end: 1000 } });
    } else if (e === 'ignore') {
      setCfgCb(null);
    }
  };

  return <Group grow>
    <Select
      onChange={onChange}
      defaultValue={defaultConfigValue(origCfg)}
      disabled={!enabled}
      data={['Exist', 'Exact', 'Range', 'ignore']}
    />
    {renderAdditionalForSelect()}
  </Group>;
};

type Props = {
  itemKey: keyof BuildItemsWithConfig,
  multipleIndex?: number,
};

export const SingleItemWithConfig = ({ itemKey, multipleIndex }: Props) => {
  const store = useContext(ItemsContext);
  if (!store) throw new Error('missing items context');
  const data = useStore(store, s => s.data);
  const item = data.provided[itemKey];

  const renderItem = (item: ItemWithConfig) => {
    return <RequiredItemComponent
      item={item.item}
      modConfigComponent={([m, cf], idx) => <div>
        <ItemModWithConfig key={idx} k={itemKey} m={m} origCfg={cf} multipleIndex={multipleIndex} />
      </div>}
    />
  };

  if (!Array.isArray(item)) {
    return renderItem(item);
  } else {
    return renderItem(item[multipleIndex!]);
  }
};
