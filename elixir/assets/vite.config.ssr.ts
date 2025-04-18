import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

// https://vite.dev/config/
export default defineConfig(() => {
  process.stdin.on("close", () => {
    process.exit(0);
  })

  process.stdin.resume();
  return {
    publicDir: "public",
    plugins: [react(), wasm(),],
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
  }
})
