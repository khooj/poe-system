import type { Meta, StoryObj } from '@storybook/react-vite';

import { Preview } from '@/pages/poe1/Preview';
import { build_data } from './items_store_data';

const meta = {
  component: Preview,
} satisfies Meta<typeof Preview>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    build_data,
  },
};
