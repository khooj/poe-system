import ReactDOMServer from "react-dom/server";
import { createInertiaApp } from "@inertiajs/react";
import './index.scss';
// @ts-ignore
// import * as bootstrap from "bootstrap";
import Layout from '@/components/Layout';
import SSRProvider from "react-bootstrap/SSRProvider";

// @ts-ignore
export function render(page) {
  return createInertiaApp({
    page,
    render: ReactDOMServer.renderToString,
    resolve: async (name) => {
      let page = await import(`./pages/${name}.tsx`);
      // @ts-ignore
      page.default.layout = page.default.layout || (page => <Layout children={page} />);
      return page;
    },
    setup: ({ App, props }) => <SSRProvider>
      <App {...props} />
    </SSRProvider>
  });
}

