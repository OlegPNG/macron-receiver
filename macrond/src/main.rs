use core::fmt;
use std::{process::{self, Command}, error::Error};

use config_file::FromConfigFile;
use log::{info, error};
use tokio_tungstenite::tungstenite::{Message, connect};
use url::Url;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct OutboundMessage {
    #[serde(rename="type")]
    message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    receiver_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    functions: Option<Vec<MacronFunction>>,
}

#[derive(Serialize, Deserialize)]
struct InboundMessage {
    #[serde(rename="type")]
    message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<usize>,
}

#[derive(Serialize, Deserialize)]
struct MacronConfig {
    server: ServerConfig,
    functions: Vec<MacronFunction>

}

#[derive(Serialize, Deserialize)]
struct ServerConfig {
    url: String,
    password: String,
    //auth_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct MacronFunction {
    id: u8,
    name: String,
    description: String,
    #[serde(skip_serializing)]
    command: String,
}

#[derive(Debug)]
struct MacronError {
    body: String
}

impl Error for MacronError {}

impl std::fmt::Display for MacronError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MacronError: {}", self.body)
    }
}

fn exec_function(id: usize, config: &MacronConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
    match config.functions.get(id) {
        Some(func) => {

            let mut cmd = Command::new(&func.command);
            cmd.output()?;
        }
        None => {
            return Err(Box::new(MacronError{ body: "Function not found".to_string() }))
        }
    }
    Ok(())

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>>{
    env_logger::init();
    info!("Starting macron daemon...");

    dotenv::dotenv().ok();

    //let server_url = std::env::var("SERVER_URL").expect("Server url not in environment.");
    //let auth_key = std::env::var("AUTH_KEY").expect("Auth key not in environtment.");
    let config_dir = match std::env::var("MACRON_CONFIG") {
        Ok(dir) => {
            dir
        }
        Err(_) => {
            let home = std::env::var("HOME").expect("No config file and cannot find HOME dir");
            home + "/.config/macron/config.toml"
        }
    };

    info!("Config file dir: {}", config_dir);

    let config = MacronConfig::from_config_file(config_dir).expect("Config file not found.");

    info!("Config functions: {}", config.functions.len());
    

    let (mut socket, _) = connect(Url::parse(&config.server.url)?)?;

    let auth_msg = OutboundMessage {
        message_type: "auth".to_string(),
        password: Some(config.server.password.clone()),
        //auth_key: config.server.auth_key.clone(),
        receiver_name: "rust".to_string(),
        functions: None,
    };

    let auth_json = serde_json::to_string(&auth_msg)?;
    socket.send(Message::Text(auth_json))?;

    let auth_response_json = socket.read()?;
    info!("Server response: {}", auth_response_json);
    let auth_response: InboundMessage = serde_json::from_str(&auth_response_json.to_string())?;

    if auth_response.message_type == "auth_success" {
        info!("Auth Success.");
    } else {
        error!("Cannot confirm authentication");
        process::exit(2);
    }

    info!("Starting loop...");
    loop {
        let msg_json = socket.read()?;
        info!("Message: {}", msg_json.to_string());

        let json: InboundMessage = serde_json::from_str(&msg_json.to_string())?;

        match json.message_type.as_str() {
            "functions" => {
                info!("Sending functions...");
                let response = OutboundMessage {
                    message_type: "functions".to_string(),
                    password: Some(config.server.password.clone()),
                    //auth_key: config.server.auth_key.clone(),
                    receiver_name: "rust".to_string(),
                    functions: Some(config.functions.clone()),
                };

                let json_response = serde_json::to_string(&response)?;
                info!("Function Response: {}", json_response);
                socket.send(Message::Text(json_response))?;
            },
            "exec" => {
                info!("Executing Function...");
                let index = json.id.unwrap_or(usize::MAX);
                exec_function(index, &config)?;
                
            }
            _ => {}
        };
    }
}
