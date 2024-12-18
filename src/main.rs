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
    persistent_credentials.get_access_token_and_account_id(&http_client, &auth_code);
    persistent_credentials.get_device_auth_and_secret(&http_client);
    dbg!(&persistent_credentials);

    /* Android temporary credentials */
    let mut android_credentials: TemporaryCredentials = TemporaryCredentials::new();
    android_credentials.get_access_token_from_device_auth(&http_client, &persistent_credentials);
    android_credentials.get_exchange_code(&http_client, "34a02cf8f4414e29b15921876da36f9a");
    dbg!(&android_credentials);

    /* Generic temporary credentials */
    let mut generic_credentials : TemporaryCredentials = TemporaryCredentials::new();
    generic_credentials.get_access_token_from_exchange_code(&http_client, &android_credentials);
    generic_credentials.get_exchange_code(&http_client, "ec684b8c687f479fadea3cb2ad83f5c6");
    dbg!(&generic_credentials);

    /* Game Launching*/

    let mut auth_password_argument = String::from("-AUTH_PASSWORD=");
    auth_password_argument.push_str(generic_credentials.exchange_code.as_str());

    let mut uid_argument = String::from("-epicuserid=");
    uid_argument.push_str(persistent_credentials.account_id.as_str());

    let command = Command::new("cmd")
        .arg("/C")  // '/C' executes the command and terminates the command shell
        .arg("start")
        .arg("/d")
        .arg("D:\\Games\\Epic Games\\Fortnite\\FortniteGame\\Binaries\\Win64")  // Path to the directory
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
    
    /*
    "D:\Games\Epic Games\Fortnite\FortniteGame\Binaries\Win64/FortniteClient-Win64-Shipping_EAC_EOS.exe"
    -obfuscationid=N8Kw52kUZsQq50886Eit-gzJOBar1g
    -AUTH_LOGIN=unused -AUTH_PASSWORD=91ec3f72c2d94c8598082d58fc007a02
    -AUTH_TYPE=exchangecode
    -epicapp=Fortnite
    -epicenv=Prod
    -EpicPortal
    -epicusername=Generic_Boi69
    -epicuserid=bff1ee7d635140ed945f69a0595526b2
    -epiclocale=en
    -epicsandboxid=fn
    -named_pipe=bff1ee7d635140ed945f69a0595526b2\Fortnite
    -fromfl=eaceos
    -caldera=eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJhY2NvdW50X2lkIjoiYmZmMWVlN2Q2MzUxNDBlZDk0NWY2OWEwNTk1NTI2YjIiLCJnZW5lcmF0ZWQiOjE3MzQ1NDMwNDYsImNhbGRlcmFHdWlkIjoiOTUyZmU2OTktZjJjMy00YjZlLTk2NzctOWRlMDMyOTcyZjkxIiwiYWNQcm92aWRlciI6IkVhc3lBbnRpQ2hlYXRFT1MiLCJub3RlcyI6IiIsInByZSI6dHJ1ZSwicGFlIjpmYWxzZSwiZmFsbGJhY2siOmZhbHNlfQ.SaiRyK-FFbzCI3TVM8RRFS0UyCu5VUsTlIMeNKPZHYcKUORdE7fZJlo0DC4zoZsPfmLNEzZxCLb_sJVPiy-m7A
    */
    
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
