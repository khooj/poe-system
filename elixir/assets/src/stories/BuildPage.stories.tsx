import type { Meta, StoryObj } from '@storybook/react-vite';

import { Build } from '@/pages/poe1/Build';
import { build_data } from './items_store_data';

const meta = {
  component: Build,
} satisfies Meta<typeof Build>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    id: "test id",
    found: build_data.data.found,
    provided: build_data.data.provided,
    processed: true
  },
};

export const Processing: Story = {
  args: {
    id: "test id",
    found: null,
    provided: build_data.data.provided,
    processed: false
  },
};
