import React, { Suspense } from "react";

import debug from "debug";
import { useProviders } from "../../api/authentication";
import { useTranslation } from "react-i18next";

/** The logger to use */
const LOGGER = debug("nrl:ui:landing:authentication");

interface AuthenticationButtonsProps {
  onClick: (provider: string) => void;
}

/**
 * Render the list of authentication buttons to display
 */
const AuthenticationButtons: React.FC<AuthenticationButtonsProps> = ({
  onClick,
}) => {
  const { t } = useTranslation();
  const providers = useProviders();

  return (
    <>
      {providers.map((provider) => (
        <button
          key={provider}
          className={`btn btn-block btn-social btn-${provider}`}
          onClick={() => onClick(provider)}
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

  const authenticate = (provider: string) => {
    LOGGER("Authenticating with provider: %o", provider);
  };

  return (
    <div>
      <h2>{t("landing.authentication.title")}</h2>
      <Suspense fallback={"loading"}>
        <AuthenticationButtons onClick={authenticate} />
      </Suspense>
    </div>
  );
};
