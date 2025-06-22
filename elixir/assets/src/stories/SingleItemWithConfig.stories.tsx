import '../index.scss';
import type { Meta, StoryObj } from '@storybook/react-vite';
import { useRef } from 'react';
import { createItemsStore, ItemsContext } from '@states/preview';

import { SingleItemWithConfig } from '@/components/SingleItemWithConfig';
import { build_data } from './items_store_data';

const meta = {
  component: SingleItemWithConfig,
  decorators: [
    (Story) => {
      const store = useRef(createItemsStore({ data: build_data.data, enabled: true })).current;

      return <ItemsContext.Provider value={store}>
        <Story />
      </ItemsContext.Provider>
    }
  ]
} satisfies Meta<typeof SingleItemWithConfig>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    itemKey: 'helmet',
  }
};

export const MultipleItemGem: Story = {
  args: {
    itemKey: 'gems',
    multipleIndex: 0
  }
};

export const MultipleItemJewel: Story = {
  args: {
    itemKey: 'jewels',
    multipleIndex: 0
  }
};

export const MultipleItemFlask: Story = {
  args: {
    itemKey: 'flasks',
    multipleIndex: 0
  }
};

