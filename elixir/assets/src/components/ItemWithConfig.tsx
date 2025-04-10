import { ModConfig } from "@bindings/domain/bindings/ModConfig";
import { isNotGem } from "@/domainutils";
import { Form } from "react-bootstrap";
import { ChangeEventHandler, useCallback, useContext } from "react";
import { Mod } from "@bindings/domain/bindings/Mod";
import { BuildItemsWithConfig } from "@bindings/domain/bindings/BuildItemsWithConfig";
import _ from "lodash";
import { ItemsContext } from "@states/preview";
import { useStore } from "zustand";

type ItemWithConfigProps = {
  k: keyof BuildItemsWithConfig,
  m: Mod,
  origCfg: ModConfig | null,
};

const ItemModWithConfig = ({ k, m, origCfg }: ItemWithConfigProps) => {
  const store = useContext(ItemsContext);
  if (!store) {
    throw new Error('missing context');
  }
  const setItemConfig = useStore(store, s => s.setItemConfig);

  const defaultConfigValue = (config: ModConfig) => {
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
    setItemConfig(k, m.stat_id, cf)
  }, [k, m, setItemConfig]);

  const renderAdditionalForSelect = () => {
    if (!origCfg) {
      return <></>
    }

    if (origCfg === 'Exist') {
      return <></>
    }

    if ("Range" in origCfg) {
      const debouncedOnChangeMin = _.debounce((e) => {
        const v = parseInt(e.target.value);
        setCfgCb({ Range: { ...origCfg.Range, start: v } });
      }, 300);
      const debouncedOnChangeMax = _.debounce((e) => setCfgCb({ Range: { ...origCfg.Range, end: parseInt(e.target.value) } }), 300);
      const { start, end } = origCfg.Range;
      return <>
        <Form.Label>Min {start}</Form.Label>
        <Form.Range
          min={0}
          max={1000}
          defaultValue={start}
          onChange={debouncedOnChangeMin}
        />
        <Form.Label>Max {end}</Form.Label>
        <Form.Range
          min={0}
          max={1000}
          defaultValue={end}
          onChange={debouncedOnChangeMax}
        />
      </>
    } else if ("Exact" in origCfg) {
      return <></>
    } else if ("Min" in origCfg) {
      const debouncedOnChangeMin = _.debounce((e) => {
        const v = parseInt(e.target.value);
        setCfgCb({ Min: v });
      }, 300);
      const min = origCfg.Min;
      return <>
        <Form.Label>Min {min}</Form.Label>
        <Form.Range
          min={0}
          max={1000}
          defaultValue={min}
          onChange={debouncedOnChangeMin}
        />
      </>
    } else if ("Max" in origCfg) {
      const debouncedOnChangeMax = _.debounce((e) => {
        const v = parseInt(e.target.value);
        setCfgCb({ Max: v });
      }, 300);
      const max = origCfg.Max;
      return <>
        <Form.Label>Max {max}</Form.Label>
        <Form.Range
          min={0}
          max={1000}
          defaultValue={max}
          onChange={debouncedOnChangeMax}
        />
      </>
    } else {
      return <p>not supported</p>
    }
  };

  const onChange: ChangeEventHandler<HTMLSelectElement> = (e) => {
    e.preventDefault();
    if (e.target.value === 'Exist') {
      // setCfgCb('Exist');
      setItemConfig(k, m.stat_id, 'Exist');
    } else if (e.target.value === 'Exact') {
      setCfgCb({ Exact: (m.current_value_int && m.current_value_int[0]) || (m.current_value_float && m.current_value_float[0]) || 0 });
    } else if (e.target.value === 'Range') {
      setCfgCb({ Range: { start: 0, end: 1000 } });
    } else if (e.target.value === 'Min') {
      // setCfgCb({ Min: 0 });
      setItemConfig(k, m.stat_id, { Min: 0 });
    } else if (e.target.value === 'Max') {
      setCfgCb({ Max: 1000 });
    } else if (e.target.value === 'ignore') {
      setCfgCb(null);
    }
  };

  return <div className='d-flex'>
    <div>{m.text}</div>
    <div className="d-flex flex-column">
      <Form.Select onChange={onChange} value={origCfg && defaultConfigValue(origCfg) || "ignore"}>
        {/* <option value="ignore" unselectable="on">Select search type</option> */}
        <option value="Exist">Exist</option>
        <option value="Exact">Exact match</option>
        <option value="Range">Range match</option>
        <option value="Min">Min</option>
        <option value="Max">Max</option>
        <option value="ignore">Ignore</option>
      </Form.Select>
      {renderAdditionalForSelect()}
    </div>
  </div>
};

type Props = {
  itemKey: keyof BuildItemsWithConfig,
};

export const ItemWithConfigComponent = ({ itemKey }: Props) => {
  const store = useContext(ItemsContext);
  if (!store) throw new Error('missing items context');
  const data = useStore(store, s => s.data);
  const item = data.provided[itemKey];

  if (!Array.isArray(item)) {
    if (isNotGem(item.item.info)) {
      return <div className='border border-primary m-2 flex-fill' style={{ fontSize: '14px' }}>
        <div className='border'>
          <span>{item.item.name}<br />{item.item.basetype}</span>
        </div>
        <div className='border'>
          <Form.Group>
            <div>{item.item.info.mods.map(([m, cf], idx) => <ItemModWithConfig key={idx} k={itemKey} m={m} origCfg={cf} />)}</div>
          </Form.Group>
        </div>
      </div >;
    } else {
      return <div className='m-2 border' style={{ fontSize: '14px' }}>
        <p>{item.item.name} {item.item.info.level}lvl/+{item.item.info.quality}%</p>
      </div>;
    }
  }

  return <div>
    <p>not rendering for now</p>
  </div>
};
