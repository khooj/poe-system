import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import wasm from 'vite-plugin-wasm';
import tsconfigPath from 'vite-tsconfig-paths';

// https://vite.dev/config/
export default defineConfig({
  publicDir: "public",
  plugins: [react(), wasm(), tsconfigPath()],
  build: {
    target: 'esnext',
    outDir: "../priv/static/assets/ssr",
    emptyOutDir: true,
    sourcemap: true,
    manifest: false,
    rollupOptions: {
      input: {
        main: "src/ssr.tsx",
      },
      output: {
        entryFileNames: "[name].mjs",
        chunkFileNames: "[name].mjs",
        assetFileNames: "[name][extname]",
        format: 'es',
      }
    }
  },
  ssr: {
    noExternal: true,
    target: "node",
  },
  css: {
    preprocessorOptions: {
      scss: {
        silenceDeprecations: ['color-functions', 'global-builtin', 'import', 'mixed-decls']
      }
    },
  },
})
