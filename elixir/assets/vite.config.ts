import { defineConfig, UserConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import wasm from 'vite-plugin-wasm';

export const conf: UserConfig = {
  publicDir: "public",
  plugins: [
    react(),
    wasm(),
    // topLevelAwait(),
  ],
  base: "/assets/",
  build: {
    target: 'esnext',
    outDir: "../priv/static/assets",
    emptyOutDir: true,
    sourcemap: true,
    manifest: false,
    commonjsOptions: {
      include: [/routes/, /node_modules/, /states/, /bindings/],
      strictRequires: "auto"
    },
    rollupOptions: {
      input: {
        main: "src/main.tsx",
      },
      output: {
        entryFileNames: "[name].js",
        chunkFileNames: "[name].js",
        assetFileNames: "[name][extname]",
      },
    },
  },
  resolve: {
    alias: [
      { find: /@bindings\/(.*)$/, replacement: '../../rust/$1.ts' },
      { find: /@states\/(.*)$/, replacement: './src/states/$1.ts' },
      { find: /@routes/, replacement: './src/routes.js' },
      { find: /@\/(.*)$/, replacement: './src/$1.tsx' },
    ],
  },
  css: {
    preprocessorOptions: {
      scss: {
        silenceDeprecations: ['color-functions', 'global-builtin', 'import', 'mixed-decls']
      }
    },
  },
};

// https://vite.dev/config/
export default defineConfig(() => {
  return conf;
})
