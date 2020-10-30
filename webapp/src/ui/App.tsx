import { BrowserRouter, Link } from "react-router-dom";

import React from "react";
import { useTranslation } from "react-i18next";

export const App: React.FC = () => {
  const { t } = useTranslation();

  return (
    <BrowserRouter>
      <div className="App">
        <header className="App-header">{t("hello")}</header>
        <Link to="/hello">World</Link>
      </div>
    </BrowserRouter>
  );
};
