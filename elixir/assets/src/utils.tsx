import Layout from '@/components/Layout';
import { useEffect } from 'react';

export const resolve = async (name: string) => {
  const pages = import.meta.glob('./pages/**/*.tsx');
  const page = await pages[`./pages/${name}.tsx`]();
  // @ts-expect-error some error
  page.default.layout = page.default.layout || (page => <Layout children={page} />);
  return page;
}

export const useLogger = (name, props) => {
  useEffect(() => {
    console.info(`Component ${name} rendered with props:`, props);
  });
};
