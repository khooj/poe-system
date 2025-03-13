import { ItemInfo } from '@bindings/domain/bindings/ItemInfo'
import { Mod } from '@bindings/domain/bindings/Mod'
import { TypedItem } from '@bindings/domain/bindings/TypedItem'
import React from 'react'

type Props = {
  item: TypedItem
}

const isWeapon = (v: ItemInfo) => {
  return v.type === "Weapon";
}

const isNotGem = (v: ItemInfo) => {
  return v.type !== "Gem";
}

const TypedItemComponent = ({ item }: Props) => {
  const renderMods = (item: ItemInfo) => {
    if (isNotGem(item)) {
      return item.mods.map(m => <><span>{m.text}</span><br /></>)
    }
  };

  return (<div>
    <p>{item.name}</p>
    <p>{item.basetype}</p>
    <p>{renderMods(item.info)}</p>
  </div>
  )
}

export default TypedItemComponent
