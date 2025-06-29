import { defineConfig, UserConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import wasm from 'vite-plugin-wasm';
import tsconfigPaths from 'vite-tsconfig-paths';

export const conf: UserConfig = {
  publicDir: "public",
  plugins: [
    react(),
    wasm(),
    tsconfigPaths(),
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
      include: [/node_modules/, /icons-react/],
      strictRequires: "auto"
    },
    rollupOptions: {
      input: {
        main: "src/main.tsx",
      },
    },
  },
  optimizeDeps: {
    include: ['@tabler/icons-react']
  }
};

// https://vite.dev/config/
export default defineConfig(conf);
