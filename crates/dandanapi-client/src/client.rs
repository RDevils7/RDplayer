use std::sync::{
    LazyLock,
    OnceLock,
    RwLock,
};

use dandanapi::{
    ClientPrelude,
    Request,
    Route,
};

use crate::{
    DandanapiError,
    secret::RequestHeaderGenerator,
};

static HEADER_GENERATOR: OnceLock<RequestHeaderGenerator> = OnceLock::new();

const OFFICIAL_BASE_URI: &str = "https://api.dandanplay.net";

#[derive(Clone, Debug)]
struct ServerConfig {
    base_uri: String,
    /// Whether to attach the signed `x-appid` / `x-signature` headers.
    /// Official DandanPlay requires them; self-hosted servers (e.g. danmu_api)
    /// authenticate via the URL token and ignore/forbid the signature.
    signed: bool,
}

static SERVER_CONFIG: LazyLock<RwLock<ServerConfig>> = LazyLock::new(|| {
    RwLock::new(ServerConfig {
        base_uri: OFFICIAL_BASE_URI.to_string(),
        signed: true,
    })
});

/// Point the danmaku client at a custom DandanPlay-compatible API server.
///
/// This is used for self-hosted servers such as `danmu_api`, where the
/// endpoint is `http://host:port/<token>` and authentication happens through
/// the URL rather than the signed `x-appid` / `x-signature` headers. When
/// `base` is empty or equals the official DandanPlay host, signed requests are
/// used.
pub fn set_base_uri(base: &str) {
    let trimmed = base.trim().trim_end_matches('/').to_string();
    let is_official = trimmed.is_empty() || trimmed.eq_ignore_ascii_case(OFFICIAL_BASE_URI);
    let mut config = SERVER_CONFIG
        .write()
        .expect("danmaku server config poisoned");
    config.base_uri = if trimmed.is_empty() {
        OFFICIAL_BASE_URI.to_string()
    } else {
        trimmed
    };
    config.signed = is_official;
}

#[derive(Clone, Debug, Default)]
pub struct DanDanClient {
    client: reqwest::Client,
}

impl DanDanClient {
    pub fn init(x_appid: String, secret: String) -> crate::Result<()> {
        let request_header_generator = RequestHeaderGenerator::new(x_appid, secret)?;
        HEADER_GENERATOR
            .set(request_header_generator)
            .map_err(|_| DandanapiError::SecretGenerationError("Already initialized".into()))
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub fn instance() -> Self {
        static INSTANCE: LazyLock<DanDanClient> = LazyLock::new(DanDanClient::default);
        INSTANCE.clone()
    }

    pub fn route<T>(&self, kind: T) -> Route<Self, T>
    where
        T: Request,
    {
        ClientPrelude::route(self, kind)
    }
}

impl ClientPrelude for DanDanClient {
    fn base_uri() -> String {
        SERVER_CONFIG
            .read()
            .expect("danmaku server config poisoned")
            .base_uri
            .clone()
    }

    fn headers(&self, path: &str) -> Option<reqwest::header::HeaderMap> {
        let signed = SERVER_CONFIG
            .read()
            .expect("danmaku server config poisoned")
            .signed;
        if signed {
            HEADER_GENERATOR.get().map(|generator| generator.header(path))
        } else {
            None
        }
    }

    fn client(&self) -> reqwest::Client {
        self.client.clone()
    }
}
