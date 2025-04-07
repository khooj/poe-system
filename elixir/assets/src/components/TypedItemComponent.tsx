import { isNotGem } from '@/domainutils'
import { Config } from '@bindings/domain/bindings/Config'
import { ItemInfo } from '@bindings/domain/bindings/ItemInfo'
import { Mod } from '@bindings/domain/bindings/Mod'
import { ModConfig } from '@bindings/domain/bindings/ModConfig'
import { TypedItem } from '@bindings/domain/bindings/TypedItem'
import { Container, Row } from 'react-bootstrap'

type Props = {
  item: TypedItem,
}

const TypedItemComponent = ({ item }: Props) => {
  const renderConfig = (cf: ModConfig | null) => {
    if (!cf) {
      return <></>
    }

    if (cf === 'Exist') {
      return <p>exist</p>
    } else if ('Exact' in cf) {
      return <p>exact: {cf.Exact}</p>
    } else if ('Range' in cf) {
      return <p>range: {cf.Range.start}-{cf.Range.end}</p>
    } else if ('Min' in cf) {
      return <p>min: {cf.Min}</p>
    } else if ('Max' in cf) {
      return <p>max: {cf.Max}</p>
    }
  };
  const renderMods = (mods: [Mod, ModConfig | null][]) => {
    return mods.map(([m, cf]) => <div className=''>{m.text} ({renderConfig(cf)})</div>)
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
    return <div className='m-2 border' style={{ fontSize: '14px' }}>
      <p>{item.name} {item.info.level}lvl/+{item.info.quality}%</p>
    </div>;
  }
}

export default TypedItemComponent
