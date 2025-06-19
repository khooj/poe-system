// Replace your-framework with the framework you are using, e.g. react-vite, nextjs, nextjs-vite, etc.
import type { Meta, StoryObj } from '@storybook/react-vite';

import RequiredItem from '../components/RequiredItemComponent';
import { RenderConfig } from '@/pages/poe1/Build';

const meta = {
  component: RequiredItem,
} satisfies Meta<typeof RequiredItem>;

export default meta;
type Story = StoryObj<typeof meta>;

export const NonSelectableOptions: Story = {
  args: {
    modConfigComponent: (mcf) => <RenderConfig cf={mcf[1]} />,
    item: {
      "basetype": "Citrine Amulet",
      "category": "Accessories",
      "id": "",
      "info": {
        "mods": [
          [
            {
              "current_value_float": null,
              "current_value_int": [
                24,
                null
              ],
              "stat_id": "additional_strength_and_dexterity",
              "text": "+(16-24) to Strength and Dexterity"
            },
            "Exist"
          ],
          [
            {
              "current_value_float": null,
              "current_value_int": [
                24,
                null
              ],
              "stat_id": "additional_dexterity",
              "text": "+24 to Dexterity"
            },
            "Exist"
          ],
          [
            {
              "current_value_float": null,
              "current_value_int": [
                29,
                null
              ],
              "stat_id": "base_fire_damage_resistance_%",
              "text": "+29% to Fire Resistance"
            },
            "Exist"
          ],
          [
            {
              "current_value_float": null,
              "current_value_int": [
                34,
                null
              ],
              "stat_id": "base_lightning_damage_resistance_%",
              "text": "+34% to Lightning Resistance"
            },
            "Exist"
          ],
          [
            {
              "current_value_float": null,
              "current_value_int": [
                52,
                null
              ],
              "stat_id": "base_maximum_life",
              "text": "+(41-55) to maximum Life"
            },
            "Exist"
          ]
        ],
        "quality": 0,
        "type": "Accessory"
      },
      "name": "New Item",
      "rarity": "rare",
      "search_basetype": false,
      "search_subcategory": false,
      "subcategory": "Amulet"
    }
  }
};

