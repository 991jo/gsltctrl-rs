use crate::jsonutil::{format_json, parse_json};
use reqwest::blocking::Client;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
mod cli;
mod jsonutil;

/// API representation
#[derive(Debug)]
struct GameServersService {
    apitoken: String,
    baseurl: String,
}

/// A single gameserver as returned by the API
#[derive(Deserialize, Debug)]
struct GameServer {
    steamid: String,
    appid: u32,
    login_token: String,

    #[serde(default = "default_str_fun")]
    memo: String,
    #[allow(dead_code)]
    is_deleted: bool,
    is_expired: bool,
    #[allow(dead_code)]
    rt_last_logon: u32,
}

fn default_str_fun() -> String {
    "".to_owned()
}

/// The result of parsing the `/GetAccountList/v1/` endpoint.
#[derive(Deserialize, Debug)]
struct GetAccountListResponse {
    servers: Vec<GameServer>,
    #[allow(dead_code)]
    is_banned: bool,
    #[allow(dead_code)]
    expires: i64,
    #[allow(dead_code)]
    actor: String,
    #[allow(dead_code)]
    last_action_time: i64,
}

/// A wrapper for responses of the Steam Web API.
#[derive(Deserialize, Debug)]
struct ResponseWrapper<T> {
    response: T,
}

/// The result of parsing the `/CreateAccount/v1/` endpoint.
#[derive(Deserialize, Debug)]
struct CreateServerResponse {
    #[allow(dead_code)]
    steamid: String,
    login_token: String,
}

/// The result of parsing the `/ResetServer/v1/` endpoint.
#[derive(Deserialize, Debug)]
struct ResetServerResponse {
    login_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct CreateAccountData {
    appid: u32,
    memo: String,
}

/// The result of parsing the `/GetAccountList/v1/` endpoint.
#[derive(Debug)]
enum ParsingResult {
    Found(String),
    NotFound,
    Expired(String),
}

impl GameServersService {
    pub fn new(apitoken: &str) -> GameServersService {
        GameServersService {
            apitoken: apitoken.to_string(),
            baseurl: "http://api.steampowered.com/IGameServersService/".to_string(),
        }
    }

    pub fn get_server_list(&self) -> GetAccountListResponse {
        let body = self.make_request("/GetAccountList/v1", None, Method::GET);

        let data: ResponseWrapper<GetAccountListResponse> = parse_json(&body);

        data.response
    }

    /// Resets the login token for the server with `steamid`.
    /// The new login token is returned.
    pub fn reset_token(&self, steamid: u64) -> String {
        let mut data = HashMap::new();
        data.insert("steamid", steamid);
        let json = format_json(data);

        let body = self.make_request("/ResetLoginToken/v1", Some(&json), Method::POST);

        let data: ResponseWrapper<ResetServerResponse> = parse_json(&body);

        data.response.login_token
    }

    /// Creates a login token with the given `appid` and `memo`.
    /// This method assumes that such a server does not exist (yet).
    ///
    /// Returns the login token.
    pub fn create_server(&self, appid: u32, memo: &str) -> String {
        let data = CreateAccountData {
            appid,
            memo: memo.to_string(),
        };

        let json = format_json(data);

        let body = self.make_request("/CreateAccount/v1/", Some(&json), Method::POST);

        let data: ResponseWrapper<CreateServerResponse> = parse_json(&body);
        data.response.login_token
    }

    /// Makes a request to the API, checks the return code for success and
    /// returns the body of the response.
    fn make_request(&self, url: &str, input_json: Option<&str>, method: Method) -> String {
        let mut path = self.baseurl.clone();
        path.push_str(url);

        let mut query: Vec<(String, String)> = Vec::new();
        query.push(("key".to_string(), self.apitoken.clone()));
        if let Some(v) = input_json {
            query.push(("input_json".to_string(), v.to_string()));
        }

        let client = Client::new();
        let request = client.request(method, path).query(&query).body("");

        let response = match request.send() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to send request: {:?}", e);
                std::process::exit(6);
            }
        };

        if !response.status().is_success() {
            eprintln!(
                "Request unsuccessful. Error code: {:?}, response: {:?}",
                response.status(),
                response.headers(),
            );
            std::process::exit(3)
        }

        match response.text() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse response text: {:?}", e);
                std::process::exit(5);
            }
        }
    }
}

fn main() {
    let args = cli::parse_args();
    let apitoken = get_apitoken();

    let service = GameServersService::new(&apitoken);

    handle_server(service, args.appid, &args.memo);
}

fn get_apitoken() -> String {
    match env::var("GSLTCTRL_TOKEN") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Could not find the GSLTCTRL_TOKEN environment variable or the the variable is not a Unicode string.");
            eprintln!("Set the GSLTCTRL_TOKEN environment variable to your API token.");
            std::process::exit(1);
        }
    }
}

fn handle_server(service: GameServersService, appid: u32, memo: &str) {
    let servers = service.get_server_list().servers;
    let result = parse_server_list(&servers, appid, memo);

    let token = match result {
        ParsingResult::Found(token) => token,
        ParsingResult::NotFound => service.create_server(appid, memo),
        ParsingResult::Expired(steamid) => {
            let steamid = match steamid.parse::<u64>() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error while parsing steamid {}: {}", steamid, e);
                    std::process::exit(5);
                }
            };

            service.reset_token(steamid)
        }
    };

    println!("{}", token);
}

/// Uses the output of the `GetAccountList` endpoint to search for the
/// server given by `appid` and `memo`.
fn parse_server_list(servers: &[GameServer], appid: u32, memo: &str) -> ParsingResult {
    for server in servers {
        if server.appid == appid && server.memo == memo {
            if server.is_expired {
                return ParsingResult::Expired(server.steamid.clone());
            } else {
                return ParsingResult::Found(server.login_token.clone());
            }
        }
    }

    ParsingResult::NotFound
}
