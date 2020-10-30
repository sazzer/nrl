import React from "react";
import { useTranslation } from "react-i18next";

export const HeaderBar: React.FC = () => {
  const { t } = useTranslation();

  return (
    <header>
      <nav className="navbar navbar-expand-lg navbar-dark bg-primary">
        <span className="navbar-brand">{t("header.title")}</span>
        <button
          className="navbar-toggler"
          type="button"
          data-toggle="collapse"
          data-target="#navbarSupportedContent"
          aria-controls="navbarSupportedContent"
          aria-expanded="false"
          aria-label={t("header.toggleNavigation")}
        >
          <span className="navbar-toggler-icon"></span>
        </button>

        <div className="collapse navbar-collapse" id="navbarSupportedContent">
          <ul className="navbar-nav ml-auto"></ul>
        </div>
      </nav>
    </header>
  );
};
