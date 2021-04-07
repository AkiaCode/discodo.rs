use serde::Serialize;
use serenity::model::id::{GuildId, UserId};

use crate::Discodo;

pub struct Http;

impl Http {
    pub async fn get(
        Discodo {
            client_data: _,
            config,
        }: &Discodo,
        router: &str,
        user_id: Option<UserId>,
        guild_id: Option<GuildId>,
    ) -> String {
        let client = reqwest::Client::new();
        let request = client.get(format!("{}{}", config.http_uri, router));

        let responsebuilder = request.header("Authorization", config.password.clone());

        let headers = if user_id.is_some() {
            responsebuilder.header("User-ID", user_id.unwrap().0)
        } else if guild_id.is_some() {
            responsebuilder.header("Guild-ID", guild_id.unwrap().0)
        } else {
            responsebuilder
        };

        let response = headers.send().await.unwrap();

        if response.status() == 403 {
            panic!("Password mismatch.")
        } else if response.status() == 404 {
            panic!("Error: {}", response.text().await.unwrap())
        }

        return response.text().await.unwrap();
    }

    pub async fn post<T>(
        Discodo {
            client_data: _,
            config,
        }: &Discodo,
        router: &str,
        user_id: Option<UserId>,
        guild_id: Option<GuildId>,
        json: Option<&T>,
    ) -> reqwest::Response
    where
        T: Serialize + ?Sized,
    {
        let client = reqwest::Client::new();
        let request = client.post(format!("{}{}", config.http_uri, router));

        let responsebuilder = request.header("Authorization", config.password.clone());

        let headers = if user_id.is_some() {
            responsebuilder.header("User-ID", user_id.unwrap().0)
        } else if guild_id.is_some() {
            responsebuilder.header("Guild-ID", guild_id.unwrap().0)
        } else {
            panic!("NEED HEADER");
        };

        let response = if json.is_some() {
            headers.json(json.unwrap()).send().await.unwrap()
        } else {
            headers.send().await.unwrap()
        };

        if response.status() == 403 {
            panic!("Password mismatch.")
        } else if response.status() == 404 {
            panic!("Error: {}", response.text().await.unwrap())
        }

        return response;
    }
}
