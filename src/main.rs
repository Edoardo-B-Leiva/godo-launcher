mod auth;

use colog;
use log::*;
use std::env::args;
use std::io::Write;
use std::process::{exit, Command};

use crate::auth::*;

fn main() {
    /* Initial initializations */
    {
        colog::init();
        // TODO: Set log level to display nothing by default
    }

    /* Command line arg fetching and handling */
    let command_line_arguments: Vec<String> = args().collect();
    {
        /* Help page */
        if command_line_arguments.iter().any(|a| a == "--help") {
            // TODO: Complete help message
            let help_page: &str = "
=== godo-launcher help page===\n
`--verbose`, `-v`, `-vv`\t| Enables debug logging.
`--debug`, `-d`\t\t\t| Enables verbose logging.
`--help`, `-h`\t\t\t| Prints this page.
";

            println!("{help_page}");
            exit(0);
        }

        /* LogLevel modifiers */
        if command_line_arguments
            .iter()
            .any(|a| a == "--verbose" || a == "-v" || a == "-vv")
        {
            // TODO: Change Log Level to info and warn
        }

        if command_line_arguments.iter().any(|a| a == "--debug") {
            warn!("Debug info enabled!");
            // TODO: Change Log Level to debug
        }
    }

    /* Authentication Process */

    //1. get access token 1 from authcode
    //2. get persistent credentials from access token 1
    //3. get access token 2 from persistent credentials
    //4. get android exchange code from access token and persistent credentials

    let http_client_builder = reqwest::blocking::ClientBuilder::new()
        .https_only(true)
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ));

    let http_client = http_client_builder
        .build()
        .unwrap_or(reqwest::blocking::Client::new());

    // Authentication code from user input
    info!("Insert Authentication Code: ");
    std::io::stdout().flush().unwrap();

    // Getting temporal info to create persistent login info
    let mut auth_code: String = "".to_string();
    let _ = std::io::stdin().read_line(&mut auth_code).unwrap();

    info!("Generating persistent credentials");
    let persistent_credentials = PersistentCredentials::fetch(&http_client, &auth_code).unwrap();
    info!("New persistent credentials: {:#?}", &persistent_credentials);

    // Generating Android access token and exchange code from persistent_credentials

    info!("Generating Android exchange code from persistent credentials");
    let android_token = AccessToken::from_persistent_credentials(
        &http_client,
        &persistent_credentials,
        ClientType::ANDROID,
    )
    .unwrap();
    info!("Android Access Token: {:#?}", &android_token);

    let android_exchange_code = ExchangeCode::from_persistent_credentials(
        &http_client,
        &android_token,
        ClientType::ANDROID,
    )
    .unwrap();
    info!("Android Exchange Code: {:#?}", &android_exchange_code);

    // Generating generic access_token and exchange code from android exchange code

    info!("Generating generic access token");
    let generic_access_token =
        AccessToken::from_exchange_code(&http_client, &android_exchange_code).unwrap();
    info!("Generic Access Token: {:#?}", &generic_access_token);

    info!("Generating generic exchange code");
    let generic_exchange_code = ExchangeCode::from_persistent_credentials(
        &http_client,
        &generic_access_token,
        ClientType::GENERIC,
    )
    .unwrap();
    info!("Generated exchange code: {:#?}", &generic_exchange_code);

    /* Game Launching*/
    info!("Starting game...");
    let mut auth_password_argument = String::from("-AUTH_PASSWORD=");
    auth_password_argument.push_str(generic_exchange_code.0.as_str());

    let mut uid_argument = String::from("-epicuserid=");
    uid_argument.push_str(persistent_credentials.account_id.as_str());

    let command = Command::new("cmd")
        .arg("/C") // '/C' executes the command and terminates the command shell
        .arg("start")
        .arg("/d")
        .arg("D:\\Games\\Epic Games\\Fortnite\\FortniteGame\\Binaries\\Win64") // Path to the directory
        .arg("FortniteLauncher.exe") // The executable
        .arg("-AUTH_LOGIN=unused")
        .arg(&auth_password_argument)
        .arg("-AUTH_TYPE=exchangecode")
        .arg("-epicapp=Fortnite")
        .arg("-epicenv=Prod")
        .arg("-EpicPortal")
        .arg("-steamimportavailable")
        .arg(&uid_argument)
        .arg("-epicsandboxid=fn")
        .spawn();

    match command {
        Ok(mut child) => {
            // Optionally, you can wait for the process to complete
            let status = child.wait().expect("Failed to wait on child");
            info!("Command executed with status: {}", status);
        }
        Err(e) => {
            error!("Error executing command: {}", e);
            exit(1);
        }
    }
}
