import type { Meta, StoryObj } from '@storybook/react-vite';

import { Index } from '@/pages/poe1/Index';
import { build_data } from './items_store_data';

const meta = {
  component: Index,
} satisfies Meta<typeof Index>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    build_ids: []
  },
};
