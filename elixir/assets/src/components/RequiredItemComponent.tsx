import { isNotGem } from '@/domainutils'
import { Mod } from '@bindings/domain/bindings/Mod'
import { ModConfig } from '@bindings/domain/bindings/ModConfig'
import { RequiredItem } from '@bindings/domain/bindings/RequiredItem'
import { JSX } from 'react'

type Props = {
  item: RequiredItem,
  modConfigComponent: ([m, cf]: [Mod, ModConfig | null], idx?: number) => JSX.Element,
}

const RequiredItemComponent = ({ item, modConfigComponent }: Props) => {
  const renderMods = (mods: [Mod, ModConfig | null][]) => {
    return mods.map((m, idx) => <div className='d-flex'>
      <div>{m[0].text}</div>
      {modConfigComponent(m, idx)}
    </div>);
  };

  if (isNotGem(item.info)) {
    return <div className='border border-primary m-2 flex-fill' style={{ fontSize: '14px' }}>
      <div className='border'>
        <span>{item.name}<br />{item.basetype}</span>
      </div>
      <div className='border'>
        <div>{renderMods(item.info.mods)}</div>
      </div>
    </div >;
  } else {
    if (item.subcategory === 'Empty') {
      return <></>
    }

    return <div className='m-2 border' style={{ fontSize: '14px' }}>
      <p>{item.name} {item.info.level}lvl/+{item.info.quality}%</p>
    </div>;
  }
}

export default RequiredItemComponent
