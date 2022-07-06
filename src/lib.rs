use reqwest::{self, Client, Method, Response};
use url::Url;

pub struct AppInfo<'a> {
    pub app_name: &'a str,
    pub api_domain: Url,
    pub website_domain: Url,
    pub api_gateway_path: &'a str,
    pub api_base_path: &'a str,
    pub website_base_path: &'a str,
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
    pub connection_uri: Url,
    pub api_key: &'a str,
}

impl Default for Connection<'_> {
    fn default() -> Self {
        Self {
            connection_uri: Url::parse("http://127.0.0.1:3567").unwrap(),
            api_key: "",
        }
    }
}

pub struct Supertokens<'a> {
    pub app_info: AppInfo<'a>,
    pub connection: Connection<'a>,
    pub recipe_list: Vec<Box<dyn Recipe>>,
    pub telemetry: Option<bool>,
}

pub trait Recipe {}

impl<'a> Supertokens<'a> {
    async fn request(self, method: Method, path: &str) -> Result<Response, reqwest::Error> {
        let client = Client::new();

        let mut uri = self.connection.connection_uri.clone();
        uri.set_path(self.app_info.api_base_path);
        uri = uri.join(path).unwrap();

        client
            .request(method, uri)
            .header("api-key", self.connection.api_key)
            .send()
            .await
    }

    pub async fn apiversion(self) -> Result<String, reqwest::Error> {
        self.request(Method::GET, "/apiversion").await?.text().await
    }

    pub async fn hello(self) -> Result<String, reqwest::Error> {
        self.request(Method::GET, "/hello").await?.text().await
    }
}

#[cfg(test)]
mod tests {
    use crate::{AppInfo, Connection, Supertokens};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
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
            telemetry: Some(false),
        };

        let result = supertokens
            .apiversion()
            .await
            .expect("SuperTokens connection failed");

        assert!(!result.is_empty())
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
            telemetry: Some(false),
        };

        let result = supertokens
            .hello()
            .await
            .expect("SuperTokens connection failed");

        assert_eq!(result.trim(), String::from("Hello"));
    }
}
