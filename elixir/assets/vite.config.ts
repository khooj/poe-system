import { defineConfig, UserConfig, searchForWorkspaceRoot } from 'vite'
import react from '@vitejs/plugin-react-swc'
import tsconfigPaths from 'vite-tsconfig-paths';

export const conf: UserConfig = {
  publicDir: "public",
  plugins: [
    react(),
    tsconfigPaths(),
  ],
  base: "/",
  server: {
    // hmr: {
    //   clientPort: 5173,
    // },
    fs: {
      allow: [
        searchForWorkspaceRoot(process.cwd()),
        '../../rust'
      ]
    }
  },
  build: {
    target: 'esnext',
    outDir: "../priv/static",
    emptyOutDir: true,
    sourcemap: false,
    manifest: "manifest.json",
    commonjsOptions: {
      strictRequires: "auto"
    },
    rollupOptions: {
      input: {
        main: "src/main.tsx",
      },
    },
  }
};

// https://vite.dev/config/
export default defineConfig(({ command }) => {
  const isDev = command !== "build";

  const localConf = { ...conf };
  if (isDev) {
    localConf.base = "/assets";
  }

  return localConf;
});
