// Replace your-framework with the framework you are using, e.g. react-vite, nextjs, nextjs-vite, etc.
import type { Meta, StoryObj } from '@storybook/react-vite';

import { StoredItemComponent as StoredItem } from '../components/StoredItemComponent';
import { build_data } from './items_store_data';

const meta = {
  component: StoredItem,
} satisfies Meta<typeof StoredItem>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    item: build_data.data.found.amulet!,
  },
};
