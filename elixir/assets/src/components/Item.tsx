import { isNotGem } from '@/domainutils'
import { Mod } from '@bindings/domain/bindings/Mod'
import { StoredItem } from '@bindings/domain/bindings/StoredItem'
import { StoredMod } from '@bindings/domain/bindings/StoredMod'
import { Flex, Stack } from '@mantine/core'
import { JSX } from 'react'
import classes from './Item.module.css';
import cx from 'clsx';
import { StoredItemInfo } from '@bindings/domain/bindings/StoredItemInfo'

type Props = {
  item: StoredItem,
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

  const renderMods = (info: StoredItemInfo) => {
    info.
    return mods.map((m, idx) => <Flex key={idx} align='center' justify='space-between'>
      <div>{renderText(m)}</div>
      {Array.isArray(m) && modConfigComponent && modConfigComponent(m, idx)}
    </Flex>);
  };

  let rarityColor = 'item-normal';
  if (item.rarity === 'magic') {
    rarityColor = 'item-magic';
  } else if (item.rarity === 'rare') {
    rarityColor = 'item-rare';
  } else if (item.rarity === 'unique') {
    rarityColor = 'item-unique';
  }

  rarityColor = `border-${rarityColor}`;

  if (isNotGem(item.info)) {
    return <Stack className={cx(classes.border, classes[rarityColor])}>
      <Flex justify='space-between' className={cx(classes['border-bottom'], classes[rarityColor])}>
        <span>{item.name}<br />{item.basetype}</span>
        {itemNameComponent && itemNameComponent(item)}
      </Flex>
      <div>
        <div>{renderMods(item.info)}</div>
      </div>
    </Stack>;
  } else {
    if (item.subcategory === 'Empty') {
      return <></>
    }

    return <div className={cx(classes.border, classes['font-style'])}>
      <p>{item.basetype} {item.info.level}lvl/+{item.info.quality}%</p>
      {itemNameComponent && itemNameComponent(item)}
    </div>;
  }
}

export default Item
