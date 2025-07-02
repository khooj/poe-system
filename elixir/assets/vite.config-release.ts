import { defineConfig } from 'vite'
import conf from './vite.config.ts';

// https://vite.dev/config/
export default defineConfig((configEnv) => {
  const res = { ...conf(configEnv) };
  res.build!.outDir = "./result";
  return res;
})
