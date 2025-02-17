import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vite.dev/config/
export default defineConfig(({ command }) => {
  return {
    publicDir: "public",
    plugins: [react()],
    build: {
      outDir: "../priv/static/assets/ssr",
      emptyOutDir: true,
      sourcemap: true,
      manifest: false,
      commonjsOptions: {
        include: [/pages/, /node_modules/],
      },
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
  }
})
