use clap::{App, AppSettings, arg};
use aws_sdk_secretsmanager::{Client, Error};

async fn show_secret(client: &Client, name: &str) -> Result<(), Error> {
    
    let resp = client.get_secret_value().secret_id(name).send().await?;
    println!("Value: {}", resp.secret_string().unwrap_or("No value!"));

    Ok(())
}

async fn show_secrets(client: &Client) -> Result<(), Error> {

    let mut resp = client.list_secrets().send().await?;
    let secrets = resp.secret_list().unwrap_or_default();
    let mut num_secrets = secrets.len();

    println!("Secret names:");
    
    for name in secrets {
        println!(" {:?}", name);
    }

    while resp.next_token != None {
        resp = client.list_secrets().set_next_token(resp.next_token).send().await?;

        let secrets = resp.secret_list().unwrap_or_default();

        num_secrets += secrets.len();

        for name in secrets {
            println!(" {:#?}", name);
        }
    }

    println!("Found {} secrets", num_secrets);

    Ok(())
}


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
        )
        .get_matches();
        

    
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
        _ => unreachable!()
    }
}
