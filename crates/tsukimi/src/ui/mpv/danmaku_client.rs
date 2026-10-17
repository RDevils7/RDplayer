use std::{
    ops::Deref,
    sync::OnceLock,
};

use dandanapi_client::*;
use mutsumi::Danmaku;

use crate::ui::{
    models::SETTINGS,
    mpv::danmaku::DanmakuExt,
};

const X_APPID: &str = "e9imrhcexn";

#[derive(Clone)]
pub struct DanmakuClient(DanDanClient);

impl DanmakuClient {
    pub fn instance() -> Option<Self> {
        // Always honor the latest custom danmaku server setting so changes apply
        // on the next danmaku action without restarting the app.
        let server = SETTINGS.danmaku_server().unwrap_or_default();
        set_base_uri(&server);

        static CLIENT: OnceLock<Option<DanmakuClient>> = OnceLock::new();
        CLIENT
            .get_or_init(|| {
                if DanDanClient::init(X_APPID.to_string(), yzlm::yzlm()).is_err() {
                    tracing::warn!("DanDanClient already initialized, using existing instance...");
                }
                Some(Self(DanDanClient::instance()))
            })
            .clone()
    }
}

impl Deref for DanmakuClient {
    type Target = DanDanClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DanmakuClient {
    pub async fn search_anime_details(
        &self, keyword: String, anime_type: AnimeType,
    ) -> anyhow::Result<Vec<SearchAnimeDetails>> {
        Ok(self
            .route(SearchSearchAnime {
                params: SearchSearchAnimeParams {
                    keyword,
                    r#type: anime_type,
                    v2: true,
                },
            })
            .await?
            .animes
            .unwrap_or_default())
    }

    pub async fn search_animes(
        &self, params: SearchSearchEpisodesParams,
    ) -> anyhow::Result<Vec<SearchEpisodesAnime>> {
        Ok(self
            .route(SearchSearchEpisodes { params })
            .await?
            .animes
            .unwrap_or_default())
    }

    pub async fn search_episode(
        &self, params: SearchSearchEpisodesParams,
    ) -> anyhow::Result<Option<(i64, String)>> {
        self.auto_select_episode(params, None).await
    }

    /// Source priority used for automatic danmaku selection:
    /// tencent > iqiyi > youku > 360 (also known as 306). Lower rank = preferred.
    const SOURCE_PRIORITY: &[&str] = &["tencent", "iqiyi", "youku", "360", "306"];

    fn source_rank(title: &str) -> u32 {
        let lower = title.to_lowercase();
        if let Some(pos) = lower.rfind("from ") {
            let src = lower[pos + 5..]
                .trim()
                .trim_end_matches(|c| c == ')' || c == '】');
            if let Some(rank) = Self::SOURCE_PRIORITY
                .iter()
                .position(|s| src.starts_with(s))
            {
                return rank as u32;
            }
        }
        u32::MAX
    }

    /// Automatically pick a danmaku library for `params`: rank candidate works by
    /// source priority, then select the requested episode (1-based index) within the
    /// chosen work. Falls back to the first episode when `episode` is `None` or out
    /// of range.
    pub async fn auto_select_episode(
        &self, params: SearchSearchEpisodesParams, episode: Option<u32>,
    ) -> anyhow::Result<Option<(i64, String)>> {
        let mut animes = self.search_animes(params).await?;
        if animes.is_empty() {
            return Ok(None);
        }
        animes.sort_by_key(|a| Self::source_rank(a.anime_title.as_deref().unwrap_or_default()));

        let target = episode.filter(|e| *e > 0).map(|e| e as usize).unwrap_or(1);
        for anime in animes {
            let anime_title = anime.anime_title.clone().unwrap_or_default();
            if let Some(episodes) = anime.episodes {
                if let Some(ep) = episodes.get(target - 1) {
                    if let Some(episode_id) = ep.episode_id {
                        let episode_title = ep.episode_title.clone().unwrap_or_default();
                        let item_name = match (episode_title.is_empty(), anime_title.is_empty()) {
                            (false, false) => format!("{episode_title} - {anime_title}"),
                            (false, true) => episode_title,
                            (true, false) => anime_title.clone(),
                            (true, true) => String::new(),
                        };
                        return Ok(Some((episode_id, item_name)));
                    }
                }
            }
        }
        Ok(None)
    }

    pub async fn get_comments(&self, episode_id: i64) -> anyhow::Result<Option<Vec<Danmaku>>> {
        let response = self
            .route(CommentGetComment {
                episode_id,
                params: CommentGetCommentParams {
                    from: 0,
                    with_related: true,
                    ch_convert: 0,
                },
            })
            .await?;

        let Some(comments) = response.comments else {
            return Ok(None);
        };

        let danmaku = comments
            .into_iter()
            .map(DanmakuExt::into_danmaku)
            .filter(|danmaku| !danmaku.content.is_empty())
            .collect::<Vec<_>>();

        Ok((!danmaku.is_empty()).then_some(danmaku))
    }
}
