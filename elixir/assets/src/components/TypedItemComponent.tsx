import { Config } from '@bindings/domain/bindings/Config'
import { ItemInfo } from '@bindings/domain/bindings/ItemInfo'
import { Mod } from '@bindings/domain/bindings/Mod'
import { TypedItem } from '@bindings/domain/bindings/TypedItem'
import { Container, Row } from 'react-bootstrap'

type Props = {
  item: TypedItem,
  config?: Config,
  configDisabled?: boolean,
}

const isNotGem = (v: ItemInfo) => {
  return v.type !== "Gem";
}

const TypedItemComponent = ({ item, config, configDisabled }: Props) => {
  const renderMods = (mods: Mod[]) => {
    return mods.map(m => <div className=''>{m.text}</div>)
  };

  const renderItem = (item: TypedItem) => {
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
  };

  return renderItem(item);
}

export default TypedItemComponent
