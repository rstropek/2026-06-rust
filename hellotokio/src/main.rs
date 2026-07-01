// This sample shows how to combine three common Rust building blocks:
//   1. Asynchronous file I/O with Tokio (non-blocking reads).
//   2. Parsing untyped JSON with serde_json.
//   3. Ergonomic error handling with a custom error type built via `thiserror`.

use serde_json::Value; // `Value` is a dynamically-typed JSON tree (object, array, string, ...).
use thiserror::Error; // Derive macro that generates the `std::error::Error` boilerplate for us.
use tokio::{fs::File, io::AsyncReadExt}; // Tokio's async `File` and the `read_buf` extension trait.

// One error type that captures every way `read_and_parse` can fail.
// `#[derive(Error)]` (from thiserror) implements `std::error::Error` and `Display`,
// `Debug` is required by `Error` and lets us print the value for diagnostics.
#[derive(Error, Debug)]
enum ParseError {
    // Each `#[error("...")]` defines the `Display` text. `{0}` interpolates the wrapped value.
    // `#[from]` auto-generates a `From<std::io::Error>` conversion, so the `?` operator can
    // turn a low-level `io::Error` into a `ParseError::FileError` automatically.
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    // Same idea for the error you get when the bytes aren't valid UTF-8.
    #[error("Failed to convert to UTF-8: {0}")]
    Utf8ConversionError(#[from] std::string::FromUtf8Error),
    // ...and for JSON that doesn't parse.
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    // A domain-specific error with no wrapped source: the JSON was valid but the key was missing.
    #[error("Connection string not found in JSON")]
    ConnectionStringNotFound,
}

// `async fn` returns a *future*: nothing runs until the caller `.await`s it.
// The return type `Result<String, ParseError>` means "either the connection string, or an error".
async fn read_and_parse(file_name: &str) -> Result<String, ParseError> {
    // `.await` suspends this task until the file is open, without blocking the OS thread.
    // `?` propagates errors: on `Err`, it converts via `From` (see `#[from]` above) and returns early.
    let mut f = File::open(file_name).await?;
    /* This is what ? does
    let f_result = File::open(file_name).await;
    if let Err(e) = f_result {
        return Err(e.into());
    }
    let mut f = f_result.unwrap();
    */

    // Read the whole file into a byte buffer. `read_buf` appends the bytes it read to `buf`.
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    // Interpret the raw bytes as UTF-8 text; fails (and `?` returns) if they aren't valid UTF-8.
    let s = String::from_utf8(buf)?;

    // Parse the text into a generic JSON tree; `?` handles malformed JSON.
    let v: Value = serde_json::from_str(s.as_str())?;
    // `get` returns `Option<&Value>`: `Some` if the key exists, `None` otherwise.
    match v.get("connectionString") {
        Some(s) => Result::Ok(s.to_string()),
        None => Result::Err(ParseError::ConnectionStringNotFound),
    }
}

// `#[tokio::main]` rewrites this into a normal `fn main` that starts the Tokio runtime
// and blocks on the async body. Without it, `main` couldn't be `async`.
#[tokio::main]
async fn main() {
    // Kick off the async operation and wait for its result.
    let result = read_and_parse("config.json").await;
    // Decide what to do based on success or failure. `{}` uses the `Display` impl that
    // thiserror generated for `ParseError`, so we get our friendly error message.
    match result {
        Ok(connection_string) => println!("Connection String: {}", connection_string),
        Err(e) => eprintln!("Error: {}", e),
    }
}
