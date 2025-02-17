import axios from "axios";
import { createInertiaApp } from "@inertiajs/react";
import { hydrateRoot } from "react-dom/client";
import './index.css';

axios.defaults.xsrfHeaderName = "x-csrf-token";

createInertiaApp({
  resolve: async (name) => {
    return await import(`./pages/${name}.tsx`);
  },
  setup({ App, el, props }) {
    hydrateRoot(el, <App {...props} />);
  },
});
