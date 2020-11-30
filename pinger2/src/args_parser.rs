use clap::{App, AppSettings, Arg};
use colored::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref EXSAMPLE: String = format!(
        // raw string そのまま文字列で表示させる
        r#"{}
        # Basic
        pinger2 --pass=root --port=33066 <db_name>

        # docker
        docker run --rm -t --network=<network> \
            --user=user --pass=secret --host=<container_name>
        "#, "Example:".yellow()
    );
    // https://docs.rs/colored/2.0.0/colored/
}

pub fn new(version: &str) -> App {
    App::new("pinger2")
        .about("ping to mysql server")
        .version(version)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .arg(
            // value_name: 引数は値の名称
            Arg::with_name("host")
                .long("host")
                .short("h")
                .help("mysql server hostname")
                .takes_value(true)
                .default_value("127.0.0.1")
                .value_name("HOST")
                .env("MYSQL_HOST"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .help("mysql server port")
                .takes_value(true)
                .default_value("3306")
                .value_name("PORT")
                .env("MYSQL_PORT"),
        )
        .arg(
            Arg::with_name("user")
                .long("user")
                .short("u")
                .help("User for authentication")
                .takes_value(true)
                .default_value("root")
                .value_name("USER")
                .env("MYSQL_USERNAME"),
        )
        .arg(
            Arg::with_name("pass")
                .long("pass")
                .alias("password")
                .short("P")
                .help("Password for authentication")
                .takes_value(true)
                .env("MYSQL_PASSWORD")
                .value_name("PASS"),
        )
        .after_help((*EXSAMPLE).as_str())
}

#[test]
fn test_args_parser() {
    // https://github.com/clap-rs/clap/blob/master/CHANGELOG.md
    // バージョンがあがると、rename...
    let app = new("0.0.1").get_matches_from_safe(vec!["", ""]);

    println!("{:?}", app);
}
