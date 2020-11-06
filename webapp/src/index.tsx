import "./i18n";

import { QueryCache, ReactQueryCacheProvider } from "react-query";

import { App } from "./ui/App";
import { BrowserRouter } from "react-router-dom";
import React from "react";
import ReactDOM from "react-dom";
import { UserProvider } from "./api/users/currentUser";
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
        <UserProvider>
          <App />
        </UserProvider>
      </BrowserRouter>
    </ReactQueryCacheProvider>
  </React.StrictMode>,
  document.getElementById("root")
);

reportWebVitals(console.log);
