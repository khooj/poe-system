// Replace your-framework with the framework you are using, e.g. react-vite, nextjs, nextjs-vite, etc.
import type { Meta, StoryObj } from '@storybook/react-vite';

import { StoredItemComponent as StoredItem } from '../components/StoredItemComponent';

const meta = {
  component: StoredItem,
} satisfies Meta<typeof StoredItem>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    item: {
      "basetype": "Citrine Amulet",
      "category": "Accessories",
      "id": "c8a8f6fc6ae2f775516ea1521f51d649c8bcb3b440e9d0af1b8728991f32d3a1",

      "info": {
        "mods": [{
          "current_value_float": null,
          "current_value_int": [17, null],
          "stat_id": "additional_dexterity",
          "text": "+17 to Dexterity"
        }, {
          "current_value_float": null,
          "current_value_int": [9, 19],
          "stat_id": "attack_minimum_added_physical_damage",
          "text": "Adds 9 to 19 Physical Damage to Attacks"
        }, {
          "current_value_float": null,
          "current_value_int": [31, null],
          "stat_id": "base_maximum_life",
          "text": "+31 to maximum Life"
        }, {
          "current_value_float": null,
          "current_value_int": [22, null],
          "stat_id": "base_maximum_mana",
          "text": "+22 to maximum Mana"
        }, {
          "current_value_float": null,
          "current_value_int": [8, null],
          "stat_id": "base_fire_damage_resistance_%",
          "text": "+8% to Fire Resistance"
        }, {
          "current_value_float": null,
          "current_value_int": [19, null],
          "stat_id": "base_lightning_damage_resistance_%",
          "text": "+19% to Lightning Resistance"
        }, {
          "current_value_float": null,
          "current_value_int": [24, null],
          "stat_id": "additional_strength_and_dexterity",
          "text": "+24 to Strength and Dexterity"
        }],

        "quality": 0,
        "type": "Accessory"
      },

      "name": "Miracle Rosary",

      "price": {
        "Custom": ["fuse", 25]
      },

      "rarity": "rare",
      "subcategory": "Amulet"
    }
  },
};
