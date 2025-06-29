import type { Meta, StoryObj } from '@storybook/react-vite';

import RequiredItem from '../components/RequiredItemComponent';
import { RenderConfig } from '@/pages/poe1/Build';
import { build_data } from './items_store_data';
import { fn } from 'storybook/test';

const meta = {
  component: RequiredItem,
} satisfies Meta<typeof RequiredItem>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    modConfigComponent: fn(),
    item: build_data.data.provided.amulet.item,
  }
};


export const WithRenderConfig: Story = {
  args: {
    modConfigComponent: (mcf) => <RenderConfig cf={mcf[1]} />,
    item: build_data.data.provided.amulet.item,
  }
};

