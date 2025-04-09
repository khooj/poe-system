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
        include: [/routes/, /node_modules/, /states/, /bindings/]
      },
      rollupOptions: {
        input: {
          main: "src/main.tsx",
        },
        output: {
          entryFileNames: "[name].js",
          chunkFileNames: "[name].js",
          assetFileNames: "[name][extname]",
          manualChunks: {
            wasm: ['wasm']
          }
        }
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
  }
})
