import ReactDOMServer from "react-dom/server";
import { createInertiaApp } from "@inertiajs/react";

// @ts-ignore
export function render(page) {
  return createInertiaApp({
    page,
    render: ReactDOMServer.renderToString,
    resolve: async (name) => {
      return await import(`./pages/${name}.tsx`);
    },
    setup: ({ App, props }) => <App {...props} />,
  });
}

