use std::path::PathBuf;
use clap::{Arg, App, AppSettings, arg};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_secretsmanager::{Client, Error, Region, PKG_VERSION};

#[tokio::main]
async fn main() {
    let matches = App::new("sreacts")
        .about("A tool that does a lot of boring activities")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(
            App::new("getsecret")
            .about("Creates a new secret in the AWS Secrets Manager")
            .arg(arg!(<NAME> "Secret ARN/Name"))
        ).get_matches();

    
    match matches.subcommand() {
        Some(("getsecret", sub_matches)) => {
            let name = sub_matches.value_of("NAME").expect("required");
            let shared_config = aws_config::load_from_env().await;
            let client = Client::new(&shared_config);
            let secret = show_secret(&client, name).await;

            println!("Secret Created: {:#?}", secret);
        }
        _ => unreachable!()
    }
}

async fn show_secret(client: &Client, name: &str) -> Result<(), Error> {
    let resp = client.get_secret_value().secret_id(name).send().await?;
    println!("Value: {}", resp.secret_string().unwrap_or("No value!"));

    Ok(())
}
