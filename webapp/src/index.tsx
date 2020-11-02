import "./i18n";

import {
  QueryCache,
  ReactQueryCacheProvider,
  useMutation,
  useQuery,
  useQueryCache,
} from "react-query";

import { App } from "./ui/App";
import { BrowserRouter } from "react-router-dom";
import React from "react";
import ReactDOM from "react-dom";
import reportWebVitals from "./reportWebVitals";

const queryCache = new QueryCache({
  defaultConfig: {
    queries: {
      suspense: true,
    },
  },
});

ReactDOM.render(
  <React.StrictMode>
    <ReactQueryCacheProvider queryCache={queryCache}>
      <BrowserRouter>
        <App />
      </BrowserRouter>
    </ReactQueryCacheProvider>
  </React.StrictMode>,
  document.getElementById("root")
);

reportWebVitals(console.log);
