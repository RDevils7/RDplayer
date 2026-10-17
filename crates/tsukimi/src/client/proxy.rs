use reqwest::Client;

pub struct ReqClient;

impl ReqClient {
    /// Build the HTTP client used for Jellyfin/Emby requests.
    ///
    /// Deliberately does NOT configure any proxy in code: reqwest honors the
    /// system proxy automatically via the `http_proxy` / `https_proxy` /
    /// `no_proxy` environment variables. The user is expected to control proxy
    /// behavior at the system level (e.g. set `no_proxy` for LAN hosts) rather
    /// than have the app hardcode a proxy target.
    pub fn build() -> Client {
        Client::builder()
            .user_agent(crate::USER_AGENT.as_str())
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to initialize client")
    }
}
