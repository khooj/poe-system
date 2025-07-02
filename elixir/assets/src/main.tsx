import '@mantine/core/styles.css';
import axios from "axios";
import { createInertiaApp } from "@inertiajs/react";
import { createRoot } from "react-dom/client";
import { resolve } from './utils.tsx';
import { MantineProvider } from "@mantine/core";
import { theme } from './theme';

axios.defaults.xsrfHeaderName = "x-csrf-token";

createInertiaApp({
  progress: {
    includeCSS: false,
  },
  resolve,
  setup({ App, el, props }) {
    createRoot(el).render(
      <MantineProvider theme={theme} getStyleNonce={() => document.head.getElementsByTagName('meta')['csp-nonce'].content}>
        <App {...props} />
      </MantineProvider>
    );
  },
});
