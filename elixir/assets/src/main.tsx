import axios from "axios";
import { createInertiaApp } from "@inertiajs/react";
import { createRoot } from "react-dom/client";
import './index.css';

axios.defaults.xsrfHeaderName = "x-csrf-token";

createInertiaApp({
  resolve: async (name) => {
    return await import(`./pages/${name}.tsx`);
  },
  setup({ App, el, props }) {
    createRoot(el).render(<App {...props} />);
  },
});
