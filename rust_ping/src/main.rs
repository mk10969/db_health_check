use diesel::{Connection, ConnectionError, MysqlConnection};
use dotenv::dotenv;

use envy;
use redis::RedisError;
use serde_derive::Deserialize;

fn main() {
    dotenv().ok();
    println!("Mysql and Redis healthcheck !");

    if let Err(err) = connect_to_mysql() {
        panic!(format!("mysql connection Error: {}", err));
    }
    if let Err(err) = connect_to_redis() {
        panic!(format!("redis connection Error: {}", err));
    }
}

fn connect_to_redis() -> Result<(), RedisError> {
    let redis_option = match envy::prefixed("REDIS_").from_env::<RedisOptions>() {
        Ok(conf) => conf,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", &redis_option.format());
    let client = redis::Client::open(redis_option.format())?;
    client.get_connection()?;
    Ok(())
}

fn connect_to_mysql() -> Result<(), ConnectionError> {
    let mysql_option = match envy::prefixed("MYSQL_").from_env::<MysqlOptions>() {
        Ok(conf) => conf,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", mysql_option.format());
    MysqlConnection::establish(&mysql_option.format())?;
    Ok(())
}

trait ConnectionOption {
    fn format(&self) -> String;
}

#[derive(Deserialize, Debug)]
struct MysqlOptions {
    host: String,
    port: i32,
    username: String,
    password: String,
    database: String,
}

#[derive(Deserialize, Debug)]
struct RedisOptions {
    host: String,
    port: i32,
}

impl ConnectionOption for MysqlOptions {
    fn format(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl ConnectionOption for RedisOptions {
    fn format(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}
