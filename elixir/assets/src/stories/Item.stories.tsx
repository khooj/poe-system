import type { Meta, StoryObj } from '@storybook/react-vite';

import Item from '../components/Item';
import { RenderConfig } from '@/pages/poe1/Build';
import { build_data } from './items_store_data';
import { fn } from 'storybook/test';

const meta = {
  component: Item,
} satisfies Meta<typeof Item>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    modConfigComponent: fn(),
    item: build_data.data.provided.amulet.item,
  }
};

export const Unique: Story = {
  args: {
    modConfigComponent: fn(),
    item: build_data.data.provided.helmet.item,
  }
};

export const WithModConfigComponent: Story = {
  args: {
    modConfigComponent: (mcf) => <RenderConfig cf={mcf[1]} />,
    item: build_data.data.provided.amulet.item,
  }
};

export const WithItemNameComponent: Story = {
  args: {
    item: build_data.data.provided.amulet.item,
    itemNameComponent: () => <>Test</>
  }
};
