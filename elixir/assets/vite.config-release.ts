import { defineConfig } from 'vite'
import { conf } from './vite.config.ts';

// https://vite.dev/config/
export default defineConfig(() => {
  const res = { ...conf };
  res.build!.outDir = "./result";
  return res;
})
