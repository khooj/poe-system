import Layout from '@/components/Layout';

export const resolve = async (name: string) => {
  const pages = import.meta.glob('./pages/**/*.tsx');
  const page = await pages[`./pages/${name}.tsx`]();
  // @ts-expect-error some error
  page.default.layout = page.default.layout || (page => <Layout children={ page } />);
  return page;
};
