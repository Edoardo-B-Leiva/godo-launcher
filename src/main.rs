mod auth;

use std::process::exit;
use log::*;
use colog;
use std::env::args;
use std::io::Write;
use std::process::Command;

use crate::auth::*;

fn main() {
    /* Initial initializations */
    {
        colog::init();
        // TODO: Set log level to display nothing
    }

    /* Command line arg fetching and handling */
    let command_line_arguments: Vec<String> = args().collect();
    {
        /* Help page */
        if command_line_arguments.iter().any(|a| a == "--help") {
            // TODO: Complete help message
            let help_page : &str =
"
=== godo-launcher help page===\n
`--verbose`, `-v`, `-vv`\t| Enables debug logging.
`--debug`, `-d`\t\t\t| Enables verbose logging.
`--help`, `-h`\t\t\t| Prints this page.
";

            println!("{help_page}");
            std::process::exit(0);
        }

        /* LogLevel modifiers */
        if command_line_arguments.iter().any(|a| a == "--verbose" || a == "-v" || a == "-vv") {
            // TODO: Change Log Level to info and warn
        }

        if command_line_arguments.iter().any(|a| a == "--debug") {
            warn!("Debug info enabled!");
            // TODO: Change Log Level to debug
        }

    }

    /* Authentication Process */

    let http_client_builder = reqwest::blocking::ClientBuilder::new()
        .https_only(true)
        .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),));

    let http_client = http_client_builder.build()
        .unwrap_or(reqwest::blocking::Client::new());

    // Authentication code from user input
    print!("Insert AuthCode > ");
    std::io::stdout().flush().unwrap();
    
    // Getting temporal info to create persistent login info
    let mut auth_code: String = "".to_string();
    let _ = std::io::stdin().read_line(&mut auth_code).unwrap();

    let mut persistent_credentials: DeviceCredentials = DeviceCredentials::new();
    {
        persistent_credentials.get_access_token_and_account_id(&http_client, &auth_code);
        persistent_credentials.get_device_auth_and_secret(&http_client);
        dbg!(&persistent_credentials);
    }

    let mut login_credentials: TemporaryCredentials = TemporaryCredentials::new();
    {
        login_credentials.get_access_token(&http_client, &persistent_credentials);
        login_credentials.get_exchange_code(&http_client);
        dbg!(&login_credentials);
    }

    /* Game Launching*/

    let mut auth_password_argument = String::from("-AUTH_PASSWORD=");
    auth_password_argument.push_str(login_credentials.exchange_code.as_str());

    let mut uid_argument = String::from("-epicuserid=");
    uid_argument.push_str(persistent_credentials.account_id.as_str());

    let command = Command::new("cmd")
        .arg("/C")  // '/C' executes the command and terminates the command shell
        .arg("start")
        .arg("/d")
        .arg("D:\\Games\\Epic Games\\Fortnite\\FortniteGame\\Binaries\\Win64")  // Path to the directory
        .arg("FortniteLauncher.exe")  // The executable
        .arg("-AUTH_LOGIN=unused")
        .arg(&auth_password_argument)
        .arg("-AUTH_TYPE=exchangecode")
        .arg("-epicapp=Fortnite")
        .arg("-epicenv=Prod")
        .arg("-EpicPortal")
        .arg(&uid_argument)
        .spawn();
    
    match command {
        Ok(mut child) => {
            // Optionally, you can wait for the process to complete
            let status = child.wait().expect("Failed to wait on child");
            println!("Command executed with status: {}", status);
        }
        Err(e) => {
            eprintln!("Error executing command: {}", e);
            exit(1);
        }
    }
}
