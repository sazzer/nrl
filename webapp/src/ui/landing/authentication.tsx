import React from "react";
import debug from "debug";
import { useTranslation } from "react-i18next";

/** The logger to use */
const LOGGER = debug("nrl:ui:landing:authentication");

/**
 * The shape of the props needed for the authentiction button
 */
interface AuthenticationButtonProps {
  provider: string;
  onClick: () => void;
}

/**
 * Component for rendering a single authentication button
 * @param provider The authentication provider
 */
const AuthenticationButton: React.FC<AuthenticationButtonProps> = ({
  provider,
  onClick,
}) => {
  const { t } = useTranslation();

  return (
    <button
      className={`btn btn-block btn-social btn-${provider}`}
      onClick={onClick}
    >
      <span className={`fa fa-${provider}`}></span>{" "}
      {t(`landing.authentication.button.${provider}`)}
    </button>
  );
};

/**
 * Pane on the landing page for the authentication details.
 */
export const AuthenticationPane: React.FC = () => {
  const { t } = useTranslation();
  const providers = ["twitter", "google", "facebook"];

  const authenticate = (provider: string) => {
    LOGGER("Authenticating with provider: %o", provider);
  };

  return (
    <div>
      <h2>{t("landing.authentication.title")}</h2>
      {providers.map((provider) => (
        <AuthenticationButton
          provider={provider}
          key={provider}
          onClick={() => authenticate(provider)}
        />
      ))}
    </div>
  );
};
