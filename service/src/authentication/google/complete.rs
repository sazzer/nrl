use super::GoogleAuthenticator;
use crate::authentication::{AuthenticatedUser, CompleteAuthentication};
use async_trait::async_trait;
use jsonwebtoken::dangerous_insecure_decode;
use serde::Deserialize;
use std::collections::HashMap;

/// Default URI for completing authentication if one isn't provided.
const DEFAULT_TOKEN_URI: &str = "https://www.googleapis.com/oauth2/v4/token";

/// The details we need from the Google response.
#[derive(Debug, Deserialize)]
struct GoogleResponse {
    id_token: Option<String>,
}

/// The details we need from the Google ID Token.
#[derive(Debug, Deserialize)]
struct IdTokenClaims {
    sub: String,
    email: String,
    given_name: String,
}

// TODO: Fix up the error handling!
#[async_trait]
impl CompleteAuthentication for GoogleAuthenticator {
    /// Complete authentication with the provider
    ///
    /// # Parameters
    /// - `params` - The parameters received from the provider
    ///
    /// # Returns
    /// The details of the user that just authenticated.
    async fn complete_authentication(
        &self,
        params: HashMap<String, String>,
    ) -> Option<AuthenticatedUser> {
        let code = params.get("code");

        if code == None {
            tracing::warn!(params = ?params, "No authentication code provided");
            return None;
        }
        let code = code.unwrap();

        let token_uri = self
            .config
            .token_uri
            .clone()
            .unwrap_or_else(|| DEFAULT_TOKEN_URI.to_owned());

        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("code", code);
        params.insert("client_id", self.config.client_id.as_ref());
        params.insert("client_secret", self.config.client_secret.as_ref());
        params.insert("redirect_uri", &self.config.redirect_uri);

        let client = reqwest::Client::new();
        let response = client.post(&token_uri).form(&params).send().await.ok()?;

        if response.status() != reqwest::StatusCode::OK {
            tracing::warn!(params = ?params, response = ?response, "Failure from Google");
            return None;
        }

        tracing::debug!(params = ?params, response = ?response, "Response from Google");

        let google_result: GoogleResponse =
            serde_json::from_slice(&response.bytes().await.ok()?).ok()?;

        tracing::debug!(params = ?params, response = ?google_result, "Payload from Google");

        let id_token = dangerous_insecure_decode::<IdTokenClaims>(&google_result.id_token?).ok()?;

        Some(AuthenticatedUser {
            user_id: id_token.claims.sub.parse().ok()?,
            display_name: id_token.claims.given_name.parse().ok()?,
            email: id_token.claims.email.parse().ok(),
        })
    }
}
