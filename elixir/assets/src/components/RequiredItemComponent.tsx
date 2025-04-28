import { Mod } from '@bindings/domain/bindings/Mod'
import { ModConfig } from '@bindings/domain/bindings/ModConfig'
import { RequiredItem } from '@bindings/domain/bindings/RequiredItem'
import { JSX } from 'react'
import Item from './Item'

type Props = {
  item: RequiredItem,
  modConfigComponent: ([m, cf]: [Mod, ModConfig | null], idx?: number) => JSX.Element,
}

const RequiredItemComponent = ({ item, modConfigComponent }: Props) => {
  return <Item item={item} modConfigComponent={modConfigComponent} />
}

export default RequiredItemComponent
