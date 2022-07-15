use recipe::Recipe;
use reqwest::{self, Client, Method, RequestBuilder};
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

pub mod ingredients;
pub mod recipe;

pub struct AppInfo<'a> {
    pub app_name: &'a str,
    pub website_domain: Url,
    pub api_domain: Url,
    pub website_base_path: &'a str,
    pub api_base_path: &'a str,
    pub api_gateway_path: &'a str,
}

impl Default for AppInfo<'_> {
    fn default() -> Self {
        Self {
            app_name: "",
            api_domain: Url::parse("http://127.0.0.1:3567").unwrap(),
            website_domain: Url::parse("http://127.0.0.1:80").unwrap(),
            api_gateway_path: "",
            api_base_path: "/auth",
            website_base_path: "/auth",
        }
    }
}

pub struct Connection<'a> {
    pub uri: Url,
    pub api_key: &'a str,
}

impl Default for Connection<'_> {
    fn default() -> Self {
        Self {
            uri: Url::parse("http://127.0.0.1:3567").unwrap(),
            api_key: "",
        }
    }
}

#[derive(Deserialize)]
pub struct ApiVersions {
    pub versions: Vec<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub status: String,
    pub path: Option<String>,
}
pub struct Supertokens<'a> {
    app_info: AppInfo<'a>,
    recipe_list: &'a [Box<dyn Recipe<'a>>],
    pub connection: Connection<'a>,
    telemetry: bool,
}

impl<'a> Supertokens<'a> {
    pub fn new(
        app_info: AppInfo<'a>,
        mut connection: Connection<'a>,
        recipe_list: &'a [Box<dyn Recipe<'a>>],
        // recipe_list: Vec<Box<dyn Recipe<'a>>>,
        // cdi_version: &'a str,
        telemetry: bool,
    ) -> Self {
        connection.uri.set_path(app_info.api_base_path);

        Self {
            app_info: app_info,
            connection: connection,
            recipe_list: recipe_list,
            telemetry: telemetry,
        }
    }

    fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let client = Client::new();

        let uri = self
            .connection
            .uri
            .join(path)
            .expect("Invalid 'path' format");

        client
            .request(method, uri)
            .header("api-key", self.connection.api_key)
    }

    pub async fn apiversion(&self) -> Result<ApiVersions, reqwest::Error> {
        self.request(Method::GET, "/apiversion")
            .send()
            .await?
            .json::<ApiVersions>()
            .await
    }

    pub async fn config(&self, pid: &str) -> Result<Config, reqwest::Error> {
        let api_versions = self.apiversion().await.expect("Could not connect o CDI");

        let cdi_version = api_versions
            .versions
            .first()
            .expect("No CDI version available");

        self.request(Method::GET, "/config")
            .header("cdi-version", cdi_version.as_str())
            .query(&[("pid", pid)])
            .send()
            .await?
            .json::<Config>()
            .await
    }

    pub async fn hello(self) -> Result<String, reqwest::Error> {
        self.request(Method::GET, "/hello")
            .send()
            .await?
            .text()
            .await
    }

    pub async fn get_user_count<'b>(recipe_ids: &[&'b str]) -> u64 {
        // TODO
        10
    }

    pub async fn get_users_oldest_first<'b>(
        pagination_token: &'b str,
        limit: i32,
        recipe_ids: &[&'b str],
    ) {
        // TODO
    }

    pub async fn get_users_newest_first<'b>(
        pagination_token: &'b str,
        limit: i32,
        recipe_ids: &[&'b str],
    ) {
        // TODO
    }

    pub async fn delete_user(user_id: Uuid) {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{AppInfo, Connection, Supertokens};
    use url::Url;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_connection_uri() {
        let supertokens = Supertokens::new(
            AppInfo {
                api_base_path: "/api",
                ..Default::default()
            },
            Connection {
                api_key: "81ed5e4b-33e2-4feb-a223-b9022f3e2b91",
                uri: Url::parse("https://0.0.0.0").unwrap(),
            },
            &[],
            false,
        );

        assert_eq!(supertokens.connection.uri.as_str(), "https://0.0.0.0/api")
    }

    #[tokio::test]
    async fn apiversion() {
        let supertokens = Supertokens {
            app_info: AppInfo {
                ..Default::default()
            },
            connection: Connection {
                api_key: "81ed5e4b-33e2-4feb-a223-b9022f3e2b91",
                ..Default::default()
            },
            recipe_list: &[],
            telemetry: false,
        };

        let result = supertokens
            .apiversion()
            .await
            .expect("SuperTokens connection failed");

        assert_eq!(
            result.versions,
            vec!["2.13", "2.12", "2.7", "2.11", "2.8", "2.10", "2.9"]
        )
    }

    #[tokio::test]
    async fn test_config_file() {
        let supertokens = Supertokens::new(
            AppInfo {
                ..Default::default()
            },
            Connection {
                api_key: "81ed5e4b-33e2-4feb-a223-b9022f3e2b91",
                ..Default::default()
            },
            &[],
            false,
        );

        let result = supertokens
            .config("10512")
            .await
            .expect("SuperTokens connection failed");

        assert_eq!(result.status, "OK")
    }

    #[tokio::test]
    async fn it_connects_to_supertokens() {
        let supertokens = Supertokens {
            app_info: AppInfo {
                ..Default::default()
            },
            connection: Connection {
                ..Default::default()
            },
            recipe_list: &[],
            telemetry: false,
        };

        let result = supertokens
            .hello()
            .await
            .expect("SuperTokens connection failed");

        assert_eq!(result.trim(), String::from("Hello"));
    }
}
