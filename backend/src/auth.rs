use axum::{
    extract::{FromRequest, Request},
    BoxError,
};
use axum_extra::extract::CookieJar;
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{ARGS, CLIENT};

#[derive(Clone, Debug)]
pub struct AuthCtx {
    pub session: Option<RauthySession>,
}

#[async_trait::async_trait]
impl<S> FromRequest<S> for AuthCtx
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_headers(req.headers());
        let rauthy_session = cookies.get("RauthySession");
        let session = async move {
            if let Some(session) = rauthy_session {
                let session_info = CLIENT
                    .get(ARGS.rauthy_url.join("/auth/v1/oidc/sessioninfo").unwrap())
                    .header("Cookie", format!("RauthySession={}", session.value()))
                    .send()
                    .await?;
                let session_info = session_info.json::<RauthySessionInfo>().await?;
                let user_info = CLIENT
                    .get(
                        ARGS.rauthy_url
                            .join(&format!("/auth/v1/users/{}", session_info.user_id))
                            .unwrap(),
                    )
                    .header("Cookie", format!("RauthySession={}", session.value()))
                    .send()
                    .await?;
                let user_info = user_info.json::<RauthyUserInfo>().await?;
                Ok::<_, reqwest::Error>(Some(RauthySession {
                    info: session_info,
                    user: user_info,
                }))
            } else {
                Ok(None)
            }
        }
        .await;
        if let Err(e) = &session {
            tracing::warn!("{e:#?}");
        }

        Ok(AuthCtx {
            session: session.ok().flatten(),
        })
    }
}

#[async_trait::async_trait]
impl<S: Sync + Send> FromRequest<S> for RauthySession {
    type Rejection = (StatusCode, &'static str);
    async fn from_request(req: Request, s: &S) -> Result<Self, Self::Rejection> {
        AuthCtx::from_request(req, s)
            .await
            .and_then(|x| x.session.ok_or(()))
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    r#"{"error": "valid user session required to access this endpiont."}"#,
                )
            })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RauthySession {
    pub info: RauthySessionInfo,
    pub user: RauthyUserInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RauthySessionInfo {
    pub id: String,
    pub user_id: String,
    pub roles: String,
    pub exp: String,
    pub timeout: String,
    pub state: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct RauthyUserInfo {
    pub id: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub language: String,
    pub roles: Vec<String>,
    pub enabled: bool,
    pub email_verified: bool,
    pub password_expires: Option<u64>,
    pub created_at: u64,
    pub last_login: u64,
    pub account_type: String,
    pub webauthn_user_id: Option<String>,
    pub user_values: RauthyUserInfoUserValues,
    pub auth_provider_id: Option<String>,
    pub federation_uid: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct RauthyUserInfoUserValues {
    pub birthdate: Option<String>,
    pub phone: Option<String>,
    pub street: Option<String>,
    pub zip: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthenticationError;
impl std::fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Authentication error: invalid bearer token / API key.")
    }
}
impl std::error::Error for AuthenticationError {}
impl AuthenticationError {
    pub async fn handle(err: BoxError) -> (StatusCode, String) {
        if err.is::<AuthenticationError>() {
            (StatusCode::UNAUTHORIZED, AuthenticationError.to_string())
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {err}"))
        }
    }
}