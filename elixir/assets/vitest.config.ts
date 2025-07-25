import { defineConfig, mergeConfig } from 'vitest/config';
import { storybookTest } from '@storybook/addon-vitest/vitest-plugin';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import tsconfigPaths from 'vite-tsconfig-paths';

const dirname =
  typeof __dirname !== 'undefined' ? __dirname : path.dirname(fileURLToPath(import.meta.url));

import viteConfig from './vite.config';

export default defineConfig((configEnv) =>
  mergeConfig(
    viteConfig(configEnv),
    {
      test: {
        // Use `workspace` field in Vitest < 3.2
        projects: [{
          plugins: [
            tsconfigPaths(),
            storybookTest({
              // The location of your Storybook config, main.js|ts
              configDir: path.join(dirname, '.storybook'),
              // This should match your package.json script to run Storybook
              // The --ci flag will skip prompts and not open a browser
              // storybookScript: 'bun run storybook --ci',
            })
          ],
          maxWorkers: 2,
          minWorkers: 1,
          test: {
            isolate: true,
            // Enable browser mode
            browser: {
              enabled: true,
              // Make sure to install Playwright
              provider: 'playwright',
              headless: true,
              instances: [{ browser: 'chromium' }],
            },
            setupFiles: ['./.storybook/vitest.setup.ts'],
          },
        }],
      },
    }),
);
