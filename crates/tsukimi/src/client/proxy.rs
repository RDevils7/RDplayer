use reqwest::{
    Client,
    Proxy,
    Url,
};

pub struct ReqClient;

/// Returns `true` for hosts that should be reached directly, bypassing any
/// configured HTTP proxy. This covers loopback and private/LAN ranges, which is
/// where media servers (Jellyfin/Emby) and self-hosted danmaku servers
/// (e.g. danmu_api) usually live. Routing these through a proxy caused slow
/// first-connection timeouts that a retry would then "fix".
fn should_bypass_proxy(host: &str) -> bool {
    if host.is_empty() {
        return false;
    }
    let host = host.trim_matches(|c| c == '[' || c == ']');
    if host == "localhost" || host == "::1" || host.ends_with(".local") || host.ends_with(".lan") {
        return true;
    }
    if let Ok(ip) = host.parse::<std::net::IpAddr>() {
        if ip.is_loopback() {
            return true;
        }
        return match ip {
            std::net::IpAddr::V4(v4) => v4.is_private() || v4.is_link_local(),
            std::net::IpAddr::V6(v6) => v6.is_unique_local() || v6.is_unicast_link_local(),
        };
    }
    false
}

impl ReqClient {
    pub fn build() -> Client {
        let mut client_builder = reqwest::Client::builder()
            .user_agent(crate::USER_AGENT.as_str())
            .timeout(std::time::Duration::from_secs(30));

        if let Some(proxy) = get_proxy_settings() {
            // Mirror reqwest's own proxy parsing: try as-is, then prepend http://.
            if let Some(proxy_url) = Url::parse(&proxy)
                .or_else(|_| Url::parse(&format!("http://{proxy}")))
                .ok()
            {
                client_builder = client_builder.proxy(Proxy::custom(move |url: &Url| {
                    if should_bypass_proxy(url.host_str().unwrap_or("")) {
                        None
                    } else {
                        Some(proxy_url.clone())
                    }
                }));
            }
        }

        client_builder.build().expect("Failed to initialize client")
    }
}

pub fn get_proxy_settings() -> Option<String> {
    use std::env;
    env::var("http_proxy")
        .or_else(|_| env::var("https_proxy"))
        .ok()
}
