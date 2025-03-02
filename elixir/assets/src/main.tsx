import axios from "axios";
import { createInertiaApp } from "@inertiajs/react";
import { hydrateRoot } from "react-dom/client";
import './index.scss';
// @ts-ignore
// import * as bootstrap from "bootstrap";
import Layout from '@/components/Layout';
import SSRProvider from "react-bootstrap/SSRProvider";

axios.defaults.xsrfHeaderName = "x-csrf-token";

createInertiaApp({
  resolve: async (name) => {
    let page = await import(`./pages/${name}.tsx`);
    // @ts-ignore
    page.default.layout = page.default.layout || (page => <Layout children={page} />);
    return page;
  },
  setup({ App, el, props }) {
    hydrateRoot(el, <SSRProvider>
      <App {...props} />
    </SSRProvider>
    );
  },
});
