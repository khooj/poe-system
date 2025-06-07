import ReactDOMServer from "react-dom/server";
import { createInertiaApp } from "@inertiajs/react";
import './index.scss';
import SSRProvider from "react-bootstrap/SSRProvider";
import { resolve } from './utils.tsx';

// @ts-expect-error page any
export function render(page) {
  return createInertiaApp({
    progress: {
      includeCSS: false,
    },
    page,
    render: ReactDOMServer.renderToString,
    resolve,
    setup: ({ App, props }) => <SSRProvider>
      <App {...props} />
    </SSRProvider>
  });
}

