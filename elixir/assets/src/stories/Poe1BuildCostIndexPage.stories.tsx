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
    // invalidateSWRCache: true,
    // reloadFrame: import.meta.env.VITE_STORYBOOK_VITEST !== "1",
  },
} satisfies Meta<typeof Index>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  play: async ({ canvas }) => {
    const el = canvas.queryByText('Loading wasm bundle');
    if (el) {
      await waitFor(() => expect(el).not.toBeVisible());
    }
    await waitFor(() => canvas.getByLabelText('Path of Building data'))
  },
};

export const WrongPob: Story = {
  play: async ({ context, canvas, userEvent }) => {
    await Primary.play(context);
    await userEvent.click(canvas.getByLabelText('Path of Building data'));
    await userEvent.paste('asddsa' + pob);
    await waitFor(() => {
      expect(canvas.getByText('Please provide correct', { exact: false })).toBeVisible();
    });
  },
};

export const PobOptions: Story = {
  name: 'Pob options with active proceed',
  play: async ({ canvas, userEvent }) => {
    await waitFor(() => {
      expect(canvas.getByLabelText('Path of Building data')).toBeVisible()
    });
    await userEvent.click(canvas.getByLabelText('Path of Building data'));
    await userEvent.paste(pob);
    await waitFor(() => {
      expect(canvas.getByLabelText('Select desired itemset')).toBeVisible();
    });
    await waitFor(() => {
      expect(canvas.getByLabelText('Select desired skillset')).toBeVisible();
    });
  },
};

export const ValidationErrorAfterChangePob: Story = {
  name: 'Validation fail after pob changed to invalid',
  play: async ({ canvas, userEvent }) => {
    await waitFor(() => {
      expect(canvas.getByLabelText('Path of Building data')).toBeVisible()
    });
    await userEvent.click(canvas.getByLabelText('Path of Building data'));
    await userEvent.paste(pob);
    await waitFor(() => {
      expect(canvas.getByLabelText('Select desired itemset')).toBeVisible();
    });
    await userEvent.clear(canvas.getByLabelText('Path of Building data'));
    await userEvent.click(canvas.getByLabelText('Path of Building data'));
    await userEvent.paste('aasdasd' + pob);
    await waitFor(() => {
      expect(canvas.getByText('Please provide correct', { exact: false })).toBeVisible();
    });
  },
};
