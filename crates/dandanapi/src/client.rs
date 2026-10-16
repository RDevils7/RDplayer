use super::{
    Request,
    Route,
};

pub trait ClientPrelude: Clone + Send + 'static {
    /// Base URL of the danmaku API server. Returned as a `String` so the
    /// endpoint can be configured at runtime (e.g. a self-hosted server).
    fn base_uri() -> String;

    fn headers(&self, path: &str) -> Option<reqwest::header::HeaderMap>;

    fn client(&self) -> reqwest::Client;

    fn route<T>(&self, kind: T) -> Route<Self, T>
    where
        T: Request,
    {
        Route::new(self, kind)
    }
}
