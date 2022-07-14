use recipe::Recipe;
use reqwest::{self, Client, Method, RequestBuilder};
use serde::Deserialize;
use url::Url;

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

pub struct Supertokens<'a> {
    app_info: AppInfo<'a>,
    recipe_list: Vec<Box<dyn Recipe<'a>>>,
    pub connection: Connection<'a>,
    telemetry: bool,
    // cdi_version: &'a str,
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

impl<'a> Supertokens<'a> {
    pub fn new(
        app_info: AppInfo<'a>,
        mut connection: Connection<'a>,
        recipe_list: Vec<Box<dyn Recipe<'a>>>,
        cdi_version: &'a str,
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
    // Result<Response, reqwest::Error>
    fn request(self, method: Method, path: &str) -> RequestBuilder {
        let client = Client::new();

        let uri = self
            .connection
            .uri
            .join(path)
            .expect("Invalid 'path' format");

        client
            .request(method, uri)
            .header("api-key", self.connection.api_key)
        // .send()
        // .await
    }

    pub async fn apiversion(self) -> Result<String, reqwest::Error> {
        self.request(Method::GET, "/apiversion")
            .send()
            .await?
            .text()
            .await
    }

    pub async fn config(self, cdi_version: &str, pid: &str) -> Result<Config, reqwest::Error> {
        // 2.14
        self.request(Method::GET, "/config")
            .header("cdi-version", cdi_version)
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
}

#[cfg(test)]
mod tests {
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
            vec![],
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
            recipe_list: vec![],
            telemetry: false,
        };

        let result = supertokens
            .apiversion()
            .await
            .expect("SuperTokens connection failed");

        // assert!(!result.is_empty())
        assert_eq!(result.trim(), "help")
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
            vec![],
            false,
        );

        let result = supertokens
            .config("2.13", "10540")
            .await
            .expect("SuperTokens connection failed");

        assert_eq!(result.status, "NOT_ALLOWED")
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
            recipe_list: vec![],
            telemetry: false,
        };

        let result = supertokens
            .hello()
            .await
            .expect("SuperTokens connection failed");

        assert_eq!(result.trim(), String::from("Hello"));
    }
}
