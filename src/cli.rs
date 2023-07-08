use clap::Parser;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    long_about = "A tool to generate and renew Steam Game Server License Tokens (GSLT).

This tool will print out a valid token for the given appid and memo.
If a token already exists it returns that token.
If it was expired it gets renewed beforehand.
If no token for that appid and memo existed a new one is created.

appid and memo are given as CLI parameters.
The web API key is read from the environment variable GSLTCTRL_TOKEN.
This is done to prevent leaking the API key via the process listing."
)]
pub(crate) struct Args {
    /// the appid for which to create a token
    pub appid: u32,

    /// the memo string. Hast to be unique per appid
    pub memo: String,
}

/// Parse command-line arguments
pub(crate) fn parse_args() -> Args {
    Args::parse()
}
