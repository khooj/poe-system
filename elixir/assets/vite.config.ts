import { defineConfig, UserConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import wasm from 'vite-plugin-wasm';
import * as path from 'path';
import tsconfigPaths from 'vite-tsconfig-paths';

export const conf: UserConfig = {
  publicDir: "public",
  plugins: [
    react(),
    wasm(),
    tsconfigPaths(),
    // topLevelAwait(),
  ],
  base: "/",
  server: {
    hmr: {
      clientPort: 5173,
    },
  },
  build: {
    target: 'esnext',
    outDir: "../priv/static",
    emptyOutDir: true,
    sourcemap: false,
    manifest: "manifest.json",
    commonjsOptions: {
      include: [/routes/, /node_modules/, /states/, /bindings/],
      strictRequires: "auto"
    },
    rollupOptions: {
      input: {
        main: "src/main.tsx",
      },
    },
  },
  resolve: {
    alias: [
      { find: '@bindings', replacement: path.resolve(__dirname, '../../rust/$1.ts') },
      { find: '@states', replacement: path.resolve(__dirname, 'src/states') },
      { find: '@routes', replacement: path.resolve(__dirname, 'src/routes.js') },
      { find: '@', replacement: path.resolve(__dirname, 'src') },
    ],
  },
  css: {
    preprocessorOptions: {
      scss: {
        silenceDeprecations: ['color-functions', 'global-builtin', 'import', 'mixed-decls']
      }
    },
  },
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './vitest.setup.mjs'
  }
};

// https://vite.dev/config/
export default defineConfig(() => {
  return conf;
})
