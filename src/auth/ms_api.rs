use std::{sync::Arc, time::Duration};

use const_format::concatcp;
use dotenv_codegen::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use poem_openapi::{param::Query, ApiResponse, OpenApi};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tracing::{error, info};

use crate::{store::Environment, LOCATION};

pub struct MicrosoftAuthApi {
    client: Client,
    redirect: String,
    oauth_const: MicrosoftOauthConst,
}

impl MicrosoftAuthApi {
    pub fn new(env: Environment) -> Self {
        let oauth_const = MicrosoftOauthConst {
            auth_url: concatcp!(
                "https://login.microsoftonline.com/",
                "organizations",
                "/oauth2/v2.0/authorize"
            ),
            token_url: concatcp!(
                "https://login.microsoftonline.com/",
                "organizations",
                // dotenv!("MICROSOFT_CLIENT_TENANT"),
                "/oauth2/v2.0/token",
            ),
            id: env.ms_id,
            secret: env.ms_secret,
            tenant: env.ms_tenant,
            callback_url: concatcp!(LOCATION, "/api/microsoft/callback"),
        };

        Self {
            client: Client::new(),
            redirect: microsoft_redirect_string(&oauth_const),
            oauth_const,
        }
    }
}

#[OpenApi]
impl MicrosoftAuthApi {
    #[oai(path = "/microsoft", method = "get")]
    async fn microsoft_redirect(&self) -> OAuthRedirectResponse {
        OAuthRedirectResponse::SuccessfulRedirect(self.redirect.clone())
    }

    #[oai(path = "/microsoft/callback", method = "get")]
    async fn ms_callback_req(
        &self,
        code: Query<String>,
        session_state: Query<String>,
    ) -> OAuthCallbackResponse {
        info!("{}", session_state.0);
        self.ms_callback(code.0).await
    }

    pub async fn ms_callback(&self, code: String) -> OAuthCallbackResponse {
        #[derive(Serialize)]
        struct MicrosoftCallbackMessage<'a> {
            code: &'a str,
            client_id: &'a str,
            client_secret: &'a str,
            redirect_uri: &'a str,
            grant_type: &'a str,
            scope: &'a str,
        }

        let to_send = MicrosoftCallbackMessage {
            code: &code,
            client_id: &self.oauth_const.id,
            client_secret: &self.oauth_const.secret,
            redirect_uri: self.oauth_const.callback_url,
            grant_type: "authorization_code",
            scope: "https://graph.microsoft.com/.default",
        };

        let serialized = serde_qs::to_string(&to_send).unwrap();
        info!("its requesting time");
        self.request_user(self.oauth_const.token_url, serialized)
            .await
    }

    pub(super) async fn request_user(&self, root: &str, query: String) -> OAuthCallbackResponse {
        let req = self
            .client
            .post(root)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(query);

        let res = match req.timeout(Duration::from_secs(10)).send().await {
            Ok(it) => it,
            Err(err) => {
                error!("Error sending: {}", err);
                return OAuthCallbackResponse::AuthenticationError;
            }
        };

        match res.error_for_status_ref() {
            Ok(_res) => (),
            Err(err) => {
                error!("Error: {}\n\nBody: {}", err, res.text().await.unwrap());
                return OAuthCallbackResponse::AuthenticationError;
            }
        };

        let text = match res.text().await {
            Ok(it) => it,
            Err(err) => {
                error!("Error when getting body: {}", err);
                return OAuthCallbackResponse::AuthenticationError;
            }
        };

        let data: MicrosoftAuthResponse = match from_str(&text) {
            Ok(it) => it,
            Err(err) => {
                error!(
                    "Error when parsing microsoft data: {} \n\nData: {}",
                    err, text
                );
                return OAuthCallbackResponse::AuthenticationError;
            }
        };

        let user_url = "https://graph.microsoft.com/v1.0/me";
        let user_response = self
            .client
            .get(user_url)
            .header("Authorization", format!("Bearer {}", data.access_token))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        info!("User: {}", user_response);

        let key = DecodingKey::from_secret(&[]);
        let mut validation = Validation::new(Algorithm::HS256);
        validation.insecure_disable_signature_validation();
        validation.set_audience(&["https://graph.microsoft.com"]);

        let data = decode::<AccessToken>(&data.access_token, &key, &validation);
        info!("{:#?}", data);

        OAuthCallbackResponse::SuccessfullyAuthenticated(LOCATION.to_string())
    }
}

pub(super) fn microsoft_redirect_string(oauth_const: &MicrosoftOauthConst) -> String {
    type Api = MicrosoftAuthApi;

    #[derive(Serialize)]
    struct MicrosoftRequestOptions<'a> {
        redirect_uri: &'a str,
        client_id: &'a str,
        access_type: &'a str,
        response_type: &'a str,
        scope: &'a str,
    }

    let options = MicrosoftRequestOptions {
        redirect_uri: &oauth_const.callback_url,
        client_id: &oauth_const.id,
        access_type: "offline",
        response_type: "code",
        scope: "https://graph.microsoft.com/user.read",
    };

    let serialized = serde_qs::to_string(&options).expect("It decided to fail");
    format!("{}?{serialized}", oauth_const.auth_url)
}

#[derive(Debug, Deserialize)]
pub struct MicrosoftAuthResponse {
    pub token_type: String,
    pub scope: String,
    pub expires_in: i64,
    pub ext_expires_in: i64,
    pub access_token: String,
}

#[derive(Debug)]
struct MicrosoftOauthConst {
    auth_url: &'static str,
    token_url: &'static str,
    id: Arc<str>,
    secret: Arc<str>,
    tenant: Arc<str>,
    callback_url: &'static str,
}

#[derive(ApiResponse)]
pub enum OAuthCallbackResponse {
    /// When everything goes right and the user successfully authenticates themselves
    #[oai(status = "301")]
    SuccessfullyAuthenticated(#[oai(header = "Location")] String),
    /// When something went wrong during authentication on the server side
    #[oai(status = "500")]
    AuthenticationError,
}

#[derive(ApiResponse)]
pub enum OAuthRedirectResponse {
    #[oai(status = "302")]
    SuccessfulRedirect(#[oai(header = "Location")] String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessToken {
    pub aud: String,
    pub iss: String,
    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,
    pub acct: i64,
    pub acr: String,
    pub aio: String,
    pub amr: Vec<String>,
    #[serde(rename = "app_displayname")]
    pub app_displayname: String,
    pub appid: String,
    pub appidacr: String,
    #[serde(rename = "family_name")]
    pub family_name: String,
    #[serde(rename = "given_name")]
    pub given_name: String,
    pub idtyp: String,
    pub ipaddr: String,
    pub name: String,
    pub oid: String,
    #[serde(rename = "onprem_sid")]
    pub onprem_sid: String,
    pub platf: String,
    pub puid: String,
    pub rh: String,
    pub scp: String,
    #[serde(rename = "signin_state")]
    pub signin_state: Vec<String>,
    pub sub: String,
    #[serde(rename = "tenant_region_scope")]
    pub tenant_region_scope: String,
    pub tid: String,
    #[serde(rename = "unique_name")]
    pub unique_name: String,
    pub upn: String,
    pub uti: String,
    pub ver: String,
    pub wids: Vec<String>,
    #[serde(rename = "xms_st")]
    pub xms_st: XmsSt,
    #[serde(rename = "xms_tcdt")]
    pub xms_tcdt: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmsSt {
    pub sub: String,
}
