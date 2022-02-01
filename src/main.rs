use clap::{App, AppSettings, arg};
use aws_sdk_secretsmanager::{Client};
mod lib;

use crate::lib::random_pass;
use crate::lib::show_secret;
use crate::lib::show_secrets;

#[tokio::main]
async fn main() {


    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let matches = App::new("sreacts")
        .about("A tool that does a lot of boring activities")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(
            App::new("getsecret")
            .about("Retrive secrets from the AWS Secrets Manager")
            .arg(arg!(<NAME> "Secret ARN/Name"))
        ).subcommand(
            App::new("listsecrets")
            .about("List all AWS Secrets Manager secrets")
        ).subcommand(
            App::new("strongpass").about("Generates a strong and secure password.")
            .arg(arg!(<LEN> "Password length"))
        ).get_matches();
        

    
    match matches.subcommand() {
        Some(("getsecret", sub_matches)) => {
            let name = sub_matches.value_of("NAME").expect("required");
            let secret = show_secret(&client, name).await;
            println!("Secret Retrieved: {:#?}", secret);

        }
        Some(("listsecrets", _sub_matches)) => {
            show_secrets(&client).await.unwrap_or_default();

            println!("Done.");
        }
        Some(("strongpass", sub_matches)) => {
            let pass_len = sub_matches
            .value_of("LEN")
            .expect("required")
            .to_string()
            .parse::<usize>().unwrap();

            random_pass(pass_len);
        }
        _ => unreachable!()
    }
}
