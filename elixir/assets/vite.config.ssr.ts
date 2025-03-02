import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vite.dev/config/
export default defineConfig(({ command }) => {
  process.stdin.on("close", () => {
    process.exit(0);
  })

  process.stdin.resume();
  return {
    publicDir: "public",
    plugins: [react()],
    build: {
      outDir: "../priv/static/assets/ssr",
      emptyOutDir: false,
      sourcemap: true,
      manifest: false,
      rollupOptions: {
        input: {
          main: "src/ssr.tsx",
        },
        output: {
          entryFileNames: "[name].js",
          chunkFileNames: "[name].js",
          assetFileNames: "[name][extname]",
          format: "cjs",
        }
      }
    },
    ssr: {
      noExternal: true,
      target: "node",
    },
    resolve: {
      alias: [
        { find: /@\/(.*)$/, replacement: './src/$1.tsx'}
      ],
    },
  }
})
