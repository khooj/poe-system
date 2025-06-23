import '@mantine/core/styles.css';
// import 'bootstrap/dist/css/bootstrap.min.css';
// import '../src/index.scss';
import React, { useEffect } from 'react';
import { addons } from 'storybook/preview-api';
import { DARK_MODE_EVENT_NAME } from '@vueless/storybook-dark-mode';
import { MantineProvider, useMantineColorScheme } from '@mantine/core';
import { theme } from '../src/theme';
import { Preview } from '@storybook/react-vite';
import { initialize, mswLoader } from 'msw-storybook-addon';
import { SWRConfig, mutate } from 'swr';

initialize({
  // onUnhandledRequest: 'bypass',
  // quiet: true,
});

const channel = addons.getChannel();

function ColorSchemeWrapper({ children }: { children: React.ReactNode }) {
  const { setColorScheme } = useMantineColorScheme();
  const handleColorScheme = (value: boolean) => setColorScheme(value ? 'dark' : 'light');

  useEffect(() => {
    channel.on(DARK_MODE_EVENT_NAME, handleColorScheme);
    return () => channel.off(DARK_MODE_EVENT_NAME, handleColorScheme);
  }, [channel]);

  return children;
}

export const invalidateSWRCache = (Story, { parameters }) => {
  if (parameters.invalidateSWRCache) {
    mutate(() => true, undefined, { revalidate: false });

    return (
      <SWRConfig
        value={{
          dedupingInterval: 0,
          revalidateOnFocus: false,
          revalidateOnMount: true,
          revalidateIfStale: false,
          provider: () => new Map(),
        }}
      >
        <Story />
      </SWRConfig>
    );
  }

  return <Story />;
};

export const ReloadFrame = ({ children }) => {
  useEffect(() => {
    return () => window.location.reload();
  });

  return children;
};

export const reloadFrameImpl = (Story, { parameters }) => {
  if (parameters.reloadFrame) {
    return <ReloadFrame><Story /></ReloadFrame>
  }
  return <Story />
};

const preview: Preview = {
  parameters: {},
  loaders: [mswLoader],
  decorators: [
    (Story) => <ColorSchemeWrapper><Story /></ColorSchemeWrapper>,
    (Story) => <MantineProvider theme={theme}><Story /></MantineProvider>,
    // (Story) => <SWRConfig value={{ provider: () => new Map() }}><Story /></SWRConfig>,
    invalidateSWRCache,
    reloadFrameImpl,
  ]
};

export default preview;
