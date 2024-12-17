mod auth;

use log::*;
use colog;
use std::env::args;
use std::io::Write;
use crate::auth::DeviceCredentials;

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

    let http_client = http_client_builder.build().unwrap_or(reqwest::blocking::Client::new());

    // Authentication code from user input
    print!("Insert AuthCode > ");
    std::io::stdout().flush().unwrap();

    let mut auth_code: String = "".to_string();
    let _ = std::io::stdin().read_line(&mut auth_code).unwrap();
        
    let mut credentials : DeviceCredentials = DeviceCredentials::new();
    credentials.get_access_token_and_account_id(&http_client, &auth_code);
    credentials.get_device_auth(&http_client);
    dbg!(&credentials);
    
    /* Game Launching*/

}
