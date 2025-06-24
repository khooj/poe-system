import type { StorybookConfig } from '@storybook/react-vite';
import wasm from 'vite-plugin-wasm';
import tsconfigPaths from 'vite-tsconfig-paths';

const config: StorybookConfig = {
  core: {
    disableTelemetry: true,
    disableWhatsNewNotifications: true,
    enableCrashReports: false,
    builder: '@storybook/builder-vite',
  },
  async viteFinal(config) {
    return {
      ...config,
      plugins: [...(config.plugins || []), wasm(), tsconfigPaths()],
    }
  },
  "stories": [
    "../src/**/*.mdx",
    "../src/**/*.stories.@(js|jsx|mjs|ts|tsx)"
  ],
  "addons": [
    "@chromatic-com/storybook",
    "@storybook/addon-docs",
    "@storybook/addon-onboarding",
    "@storybook/addon-a11y",
    "@vueless/storybook-dark-mode",
    "@storybook/addon-vitest",
  ],
  "framework": {
    "name": "@storybook/react-vite",
    "options": {}
  },
  staticDirs: ['../public'],
  env: (config) => ({
    ...config,
    IN_STORYBOOK: '1'
  })
};
export default config;
