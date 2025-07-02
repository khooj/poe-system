import type { Meta, StoryObj } from '@storybook/react-vite';

import { Index } from '@/pages/poe1/Index';
import pob from './pob';
import { http, delay, HttpResponse } from 'msw';

import {
  expect,
  waitFor,
  waitForElementToBeRemoved,
} from 'storybook/test';

const meta = {
  component: Index,
  parameters: {
    invalidateSWRCache: true,
    reloadFrame: import.meta.env.VITE_STORYBOOK_VITEST !== "1",
  },
} satisfies Meta<typeof Index>;

export default meta;
type Story = StoryObj<typeof meta>;

export const LoadingWasm: Story = {
  parameters: {
    msw: {
      handlers: [
        http.get(/(rust)?.*wasm.*/, async () => {
          await delay(2000);
        }),
      ]
    }
  },
  play: async ({ canvas }) => {
    await waitFor(() => {
      expect(canvas.getByText('Loading wasm bundle')).toBeVisible();
    });
  },
};

export const ErrorLoadingWasm: Story = {
  parameters: {
    msw: {
      handlers: [
        http.get(/(rust)?.*wasm.*/i, async () => {
          return new HttpResponse(null, { status: 404 });
        }),
      ]
    }
  },
  play: async ({ canvas }) => {
    await waitFor(() => canvas.getByText(/loading error/i), { interval: 100 });
  },
};
