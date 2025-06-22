import '@mantine/core/styles.css';
// import 'bootstrap/dist/css/bootstrap.min.css';
// import '../src/index.scss';
import React, { useEffect } from 'react';
import { addons } from 'storybook/preview-api';
import { DARK_MODE_EVENT_NAME } from '@vueless/storybook-dark-mode';
import { MantineProvider, useMantineColorScheme } from '@mantine/core';
import { theme } from '../src/theme';
import { Preview } from '@storybook/react-vite';

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

const preview: Preview = {
  parameters: {},
  decorators: [
    (Story) => <ColorSchemeWrapper><Story /></ColorSchemeWrapper>,
    (Story) => <MantineProvider theme={theme}><Story /></MantineProvider>
  ]
};

export default preview;
