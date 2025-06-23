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
    reloadFrame: true
  },
  // decorators: [ReloadFrame]
} satisfies Meta<typeof Index>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    build_ids: []
  },
};

export const LoadingWasm: Story = {
  args: {
    build_ids: []
  },
  parameters: {
    msw: {
      handlers: [
        http.get(/rust.*wasm.*/, async () => {
          await delay('infinite');
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
  args: {
    build_ids: []
  },
  parameters: {
    msw: {
      handlers: [
        http.get(/rust.*wasm.*/, async () => {
          await delay(500);
          return new HttpResponse(null, { status: 404 });
        }),
      ]
    }
  },
  play: async ({ canvas }) => {
    await waitFor(() => {
      expect(canvas.getByText('Loading wasm bundle')).toBeVisible();
    });
    await waitForElementToBeRemoved(() => canvas.queryByText('Loading wasm bundle'));
    waitFor(() => canvas.queryByText('Wasm loading error', { exact: false }));
  },
};

export const WrongPob: Story = {
  args: {
    build_ids: []
  },
  play: async ({ canvas, userEvent }) => {
    await waitFor(() => {
      expect(canvas.getByLabelText('Path of Building data')).toBeVisible()
    });
    await userEvent.click(canvas.getByLabelText('Path of Building data'));
    await userEvent.paste('asddsa' + pob);
    await waitFor(() => {
      expect(canvas.getByText('Please provide correct', { exact: false })).toBeVisible();
    });
  },
};

export const PobOptions: Story = {
  args: {
    build_ids: []
  },
  play: async ({ canvas, userEvent }) => {
    await waitFor(() => {
      expect(canvas.getByLabelText('Path of Building data')).toBeVisible()
    });
    await userEvent.click(canvas.getByLabelText('Path of Building data'));
    await userEvent.paste(pob);
    await waitFor(() => {
      expect(canvas.getByLabelText('Select desired itemset')).toBeVisible();
    });
  },
};
