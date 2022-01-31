use std::path::PathBuf;
use clap::{Arg, App, AppSettings, arg};


fn main() {
    let matches = App::new("sreacts")
        .about("A tool that does a lot of boring activities")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(
            App::new("secretadd")
            .about("Creates a new secret in the AWS Secrets Manager")
            .arg(
                Arg::new("description")
                .short('d')
                .help("Secret Description")
            )
        ).get_matches();

    
    match matches.subcommand() {
        _ => unreachable!()
    }
}
