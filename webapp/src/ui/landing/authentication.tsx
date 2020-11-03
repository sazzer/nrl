import React, { Suspense } from "react";
import { useAuthenticate, useProviders } from "../../api/authentication";

import debug from "debug";
import { useTranslation } from "react-i18next";

const LOGGER = debug("nrl:ui:landing:authentication");

/**
 * Render the list of authentication buttons to display
 */
const AuthenticationButtons: React.FC = () => {
  const { t } = useTranslation();
  const providers = useProviders();
  const { authenticate } = useAuthenticate();
  LOGGER("Rendering authentication buttons: %o", providers);

  return (
    <>
      {providers.map((provider) => (
        <button
          key={provider}
          className={`btn btn-block btn-social btn-${provider}`}
          onClick={() => authenticate(provider)}
        >
          <span className={`fa fa-${provider}`}></span>{" "}
          {t(`landing.authentication.button.${provider}`)}
        </button>
      ))}
    </>
  );
};

/**
 * Pane on the landing page for the authentication details.
 */
export const AuthenticationPane: React.FC = () => {
  const { t } = useTranslation();
  LOGGER("Rendering authentication pane");

  return (
    <div>
      <h2>{t("landing.authentication.title")}</h2>
      <Suspense fallback={"loading"}>
        <AuthenticationButtons />
      </Suspense>
    </div>
  );
};
