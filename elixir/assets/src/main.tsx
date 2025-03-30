import axios from "axios";
import { createInertiaApp } from "@inertiajs/react";
import { hydrateRoot } from "react-dom/client";
import './index.scss';
import SSRProvider from "react-bootstrap/SSRProvider";
import { resolve } from './utils.tsx';

axios.defaults.xsrfHeaderName = "x-csrf-token";

createInertiaApp({
  resolve,
  setup({ App, el, props }) {
    hydrateRoot(el, <SSRProvider>
      <App {...props} />
    </SSRProvider>
    );
  },
});
