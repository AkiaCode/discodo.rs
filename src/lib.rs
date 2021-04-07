use async_trait::async_trait;
use http::Http;
use parking_lot::RwLock as PRwLock;
use request_values::{
    LoadSource, Nothing, PutSoucre, Seek, SetAutoplay, SetCrossfade, SetGapless, SetVolume, Skip,
    Unmark,
};
use response_values::{Source, SourceResponse, StatusResponse};
use serenity::{
    client::ClientBuilder as serenity_builder,
    futures::StreamExt,
    model::id::{GuildId, UserId},
};
use std::sync::Arc;
use tokio::spawn;
use url::Url;
pub mod http;
pub mod request_values;
pub mod response_values;

#[derive(Debug, Clone)]
pub struct ClientBuilder {
    pub host: String,
    pub port: usize,
    password: String,
    pub http_uri: String,
    pub ws_uri: String,
}

impl ClientBuilder {
    fn new(host: &str, port: Option<usize>, password: Option<&str>) -> Self {
        let port = match port {
            Some(some) => some,
            None => 8000,
        };

        let password = match password {
            Some(some) => some.to_string(),
            None => String::from("hellodiscodo"),
        };

        Self {
            host: host.to_string(),
            port: port.clone(),
            password: password.clone(),
            http_uri: format!("http://{}:{}/", host, port),
            ws_uri: format!("ws://{}:{}/ws/", host, port),
        }
    }
}
#[async_trait]
pub trait SerenityInit {
    async fn register_discodo(
        self,
        host: &str,
        port: Option<usize>,
        password: Option<&str>,
    ) -> Self;

    fn register_discodo_with(self, voice: Arc<Discodo>) -> Self;
}

#[async_trait]
impl SerenityInit for serenity_builder<'static> {
    async fn register_discodo(
        self,
        host: &str,
        port: Option<usize>,
        password: Option<&str>,
    ) -> Self {
        register(self, host, port, password).await
    }

    fn register_discodo_with(self, discodo: Arc<Discodo>) -> Self {
        register_with(self, discodo)
    }
}

pub async fn register<'x, 'y>(
    client_builder: serenity_builder<'x>,
    host: &str,
    port: Option<usize>,
    password: Option<&str>,
) -> serenity_builder<'y>
where
    'x: 'y,
{
    let discodo = Discodo::serenity(host, port, password).await;
    Discodo::connect(discodo.clone()).await;
    register_with(client_builder, discodo)
}

pub fn register_with(client_builder: serenity_builder, discodo: Arc<Discodo>) -> serenity_builder {
    client_builder.type_map_insert::<DiscodoKey>(discodo)
}

pub async fn get(ctx: &serenity::client::Context) -> Option<Arc<Discodo>> {
    let data = ctx.data.read().await;
    data.get::<DiscodoKey>().cloned()
}

pub struct DiscodoKey;

impl serenity::prelude::TypeMapKey for DiscodoKey {
    type Value = Arc<Discodo>;
}
#[derive(Clone, Copy, Debug, Default)]
struct ClientData {
    shard_count: u64,
    initialed: bool,
    user_id: serenity::model::id::UserId,
    guild_id: serenity::model::id::GuildId,
}

#[derive(Debug)]
pub struct Discodo {
    client_data: PRwLock<ClientData>,
    config: ClientBuilder,
}

impl Discodo {
    pub async fn serenity(host: &str, port: Option<usize>, password: Option<&str>) -> Arc<Self> {
        Arc::new(Self {
            client_data: Default::default(),
            config: ClientBuilder::new(host, port, password),
        })
    }

    pub async fn connect(discodo: Arc<Discodo>) {
        spawn(async move {
            let url = Url::parse(discodo.config.ws_uri.clone().as_str()).unwrap();
            let (ws_socket, _) = tokio_tungstenite::connect_async(url).await.unwrap();

            let (_write, mut _read) = ws_socket.split();

            // println!("{}", read.next().await.unwrap().as_ref().unwrap());
        })
        .await
        .unwrap();
    }

    pub fn get<U: Into<UserId>, G: Into<GuildId>>(&self, user_id: U, guild_id: G) {
        let mut data = self.client_data.write();

        if data.initialed {
            return;
        }

        data.guild_id = guild_id.into();
        data.user_id = user_id.into();
    }

    pub async fn status(&self) -> StatusResponse {
        let response = Http::get(self, "status", None, None).await;
        let json: StatusResponse = serde_json::from_str(&response).unwrap();

        return json;
    }

    pub async fn planner(&self) -> String {
        let response = Http::get(self, "planner", None, None).await;

        return response;
    }

    pub async fn unmark(&self, address: &str) -> String {
        let response = Http::post(
            self,
            "planner/unmark",
            None,
            None,
            Some(&Unmark {
                address: address.to_string(),
            }),
        )
        .await;

        return response.text().await.unwrap();
    }

    pub async fn unmark_all(&self) -> String {
        let response = Http::post::<Nothing>(self, "planner/unmark/all", None, None, None).await;

        return response.text().await.unwrap();
    }

    pub async fn get_source(&self, query: &str) -> SourceResponse {
        let response = Http::get(
            self,
            format!("getSource?query={}", query).as_str(),
            None,
            None,
        )
        .await;
        let json: SourceResponse = serde_json::from_str(&response).unwrap();

        return json;
    }

    pub async fn put_source(
        &self,
        user_id: UserId,
        guild_id: GuildId,
        source: Source,
    ) -> Option<String> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response = Http::post(
            self,
            "putSource",
            Some(user_id),
            Some(guild_id),
            Some(&PutSoucre { source }),
        )
        .await;

        return Some(response.text().await.unwrap());
    }

    pub async fn load_source(
        &self,
        user_id: UserId,
        guild_id: GuildId,
        query: &str,
    ) -> Option<String> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response = Http::post(
            self,
            "loadSource",
            Some(user_id),
            Some(guild_id),
            Some(&LoadSource {
                query: query.to_owned(),
            }),
        )
        .await;

        return Some(response.text().await.unwrap());
    }

    pub async fn set_volume(&self, user_id: UserId, guild_id: GuildId, volume: f32) -> Option<u16> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response = Http::post(
            self,
            "setVolume",
            Some(user_id),
            Some(guild_id),
            Some(&SetVolume { volume }),
        )
        .await;

        return Some(response.status().as_u16());
    }

    pub async fn set_crossfade(
        &self,
        user_id: UserId,
        guild_id: GuildId,
        crossfade: f32,
    ) -> Option<u16> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response = Http::post(
            self,
            "setCrossfade",
            Some(user_id),
            Some(guild_id),
            Some(&SetCrossfade { crossfade }),
        )
        .await;

        return Some(response.status().as_u16());
    }

    pub async fn set_gapless(
        &self,
        user_id: UserId,
        guild_id: GuildId,
        gapless: f32,
    ) -> Option<u16> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response = Http::post(
            self,
            "setGapless",
            Some(user_id),
            Some(guild_id),
            Some(&SetGapless { gapless }),
        )
        .await;

        return Some(response.status().as_u16());
    }

    pub async fn set_autoplay(
        &self,
        user_id: UserId,
        guild_id: GuildId,
        autoplay: bool,
    ) -> Option<u16> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response = Http::post(
            self,
            "setAutoplay",
            Some(user_id),
            Some(guild_id),
            Some(&SetAutoplay { autoplay }),
        )
        .await;

        return Some(response.status().as_u16());
    }

    pub async fn seek(&self, user_id: UserId, guild_id: GuildId, offset: f32) -> Option<u16> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response = Http::post(
            self,
            "seek",
            Some(user_id),
            Some(guild_id),
            Some(&Seek { offset }),
        )
        .await;

        return Some(response.status().as_u16());
    }

    pub async fn skip(&self, user_id: UserId, guild_id: GuildId, offset: f32) -> Option<u16> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response = Http::post(
            self,
            "skip",
            Some(user_id),
            Some(guild_id),
            Some(&Skip { offset }),
        )
        .await;

        return Some(response.status().as_u16());
    }

    pub async fn pause(&self, user_id: UserId, guild_id: GuildId) -> Option<u16> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response =
            Http::post::<Nothing>(self, "pause", Some(user_id), Some(guild_id), None).await;

        return Some(response.status().as_u16());
    }

    pub async fn resume(&self, user_id: UserId, guild_id: GuildId) -> Option<u16> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response =
            Http::post::<Nothing>(self, "resume", Some(user_id), Some(guild_id), None).await;

        return Some(response.status().as_u16());
    }

    pub async fn shuffle(&self, user_id: UserId, guild_id: GuildId) -> Option<String> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response =
            Http::post::<Nothing>(self, "shuffle", Some(user_id), Some(guild_id), None).await;

        return Some(response.text().await.unwrap());
    }

    pub async fn remove(&self, user_id: UserId, guild_id: GuildId) -> Option<String> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response =
            Http::post::<Nothing>(self, "remove", Some(user_id), Some(guild_id), None).await;

        return Some(response.text().await.unwrap());
    }

    pub async fn state(&self, user_id: UserId, guild_id: GuildId) -> Option<String> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response =
            Http::get(self, "state", Some(user_id), Some(guild_id)).await;

        return Some(response);
    }

    pub async fn queue(&self, user_id: UserId, guild_id: GuildId) -> Option<String> {
        if user_id.0 == 0 || guild_id.0 == 0 {
            return None;
        }

        let response = Http::get(self, "queue", Some(user_id), Some(guild_id)).await;

        return Some(response);
    }

    pub async fn set_filter(&self, _user_id: UserId, _guild_id: GuildId) {
        unimplemented!(
            "https://github.com/kijk2869/discodo/blob/master/docs/server/restful.md#post-setfilter"
        );
    }
}
