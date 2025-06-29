import type { Meta, StoryObj } from '@storybook/react-vite';

import Layout from '@/components/Layout';

const meta = {
  component: Layout,
} satisfies Meta<typeof Layout>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    children: <></>
  },
};
