import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vite.dev/config/
export default defineConfig(({ command }) => {
  const isDev = command !== "build";

  if (isDev) {
    process.stdin.on("close", () => {
      process.exit(0);
    })

    process.stdin.resume();
  }

  return {
    publicDir: "public",
    plugins: [react()],
    build: {
      outDir: "../priv/static",
      emptyOutDir: true,
      sourcemap: isDev,
      manifest: false,
      commonjsOptions: {
        include: [/pages/, /node_modules/],
      },
      rollupOptions: {
        external: [
          "/vite.svg"
        ],
        input: {
          main: "src/main.tsx",
        },
        output: {
          entryFileNames: "assets/[name].js",
          chunkFileNames: "assets/[name].js",
          assetFileNames: "assets/[name][extname]"
        }
      }
    },
  }
})
