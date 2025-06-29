import ReactDOMServer from "react-dom/server";
import { createInertiaApp } from "@inertiajs/react";
import { resolve } from './utils.tsx';
import { MantineProvider } from "@mantine/core";
import { theme } from './theme';

// @ts-expect-error page any
export function render(page) {
  return createInertiaApp({
    progress: {
      includeCSS: false,
    },
    page,
    render: ReactDOMServer.renderToString,
    resolve,
    setup: ({ App, props }) => <MantineProvider theme={theme}>
      <App {...props} />
    </MantineProvider>
  });
}

