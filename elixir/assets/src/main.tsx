import axios from "axios";
import { createInertiaApp } from "@inertiajs/react";
import { hydrateRoot } from "react-dom/client";
import './index.scss';
import SSRProvider from "react-bootstrap/SSRProvider";
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
    hydrateRoot(el, <SSRProvider>
      <MantineProvider theme={theme}>
        <App {...props} />
      </MantineProvider>
    </SSRProvider>
    );
  },
});
