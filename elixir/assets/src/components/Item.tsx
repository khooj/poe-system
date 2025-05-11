import { isNotGem } from '@/domainutils'
import { Mod } from '@bindings/domain/bindings/Mod'
import { ModConfig } from '@bindings/domain/bindings/ModConfig'
import { RequiredItem } from '@bindings/domain/bindings/RequiredItem'
import { StoredItem } from '@bindings/domain/bindings/StoredItem'
import { StoredMod } from '@bindings/domain/bindings/StoredMod'
import { JSX } from 'react'

type PropItem = RequiredItem | StoredItem;
type Props = {
  item: PropItem,
  modConfigComponent?: ([m, cf]: [Mod, ModConfig | null], idx?: number) => JSX.Element,
  itemNameComponent?: (item: PropItem) => JSX.Element,
}

const Item = ({ item, modConfigComponent, itemNameComponent }: Props) => {
  const renderText = (m: [Mod, ModConfig | null] | StoredMod) => {
    if (Array.isArray(m)) {
      return m[0].text;
    } else {
      return m.text;
    }
  };

  const renderMods = (mods: [Mod, ModConfig | null][] | StoredMod[]) => {
    return mods.map((m, idx) => <div className='d-flex align-items-center'>
      <div>{renderText(m)}</div>
      {Array.isArray(m) && modConfigComponent && modConfigComponent(m, idx)}
    </div>);
  };

  let rarityColor = 'item-normal';
  if (item.rarity === 'magic') {
    rarityColor = 'item-magic';
  } else if (item.rarity === 'rare') {
    rarityColor = 'item-rare';
  } else if (item.rarity === 'unique') {
    rarityColor = 'item-unique';
  }

  if (isNotGem(item.info)) {
    return <div className={`border m-2 ${rarityColor}`}>
      <div className={`border-bottom d-flex justify-content-between`}>
        <span>{item.name}<br />{item.basetype}</span>
        {itemNameComponent && itemNameComponent(item)}
      </div>
      <div className=''>
        <div>{renderMods(item.info.mods)}</div>
      </div>
    </div >;
  } else {
    if (item.subcategory === 'Empty') {
      return <></>
    }

    return <div className={`m-2 border`} style={{ fontSize: '14px' }}>
      <p>{item.name} {item.info.level}lvl/+{item.info.quality}%</p>
    </div>;
  }
}

export default Item
