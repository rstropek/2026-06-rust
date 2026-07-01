use serde_json::Value;
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Error, Debug)]
enum ParseError {
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Failed to convert to UTF-8: {0}")]
    Utf8ConversionError(#[from] std::string::FromUtf8Error),
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Connection string not found in JSON")]
    ConnectionStringNotFound,
}

async fn read_and_parse(file_name: &str) -> Result<String, ParseError> {
    let mut f = File::open(file_name).await?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let s = String::from_utf8(buf)?;

    let v: Value = serde_json::from_str(s.as_str())?;
    match v.get("connectionString") {
        Some(s) => Result::Ok(s.to_string()),
        None => Result::Err(ParseError::ConnectionStringNotFound),
    }
}

#[tokio::main]
async fn main() {
    let result = read_and_parse("config.json").await;
    match result {
        Ok(connection_string) => println!("Connection String: {}", connection_string),
        Err(e) => eprintln!("Error: {}", e),
    }
}
