# GSLTCTRL-RS

This is a simple cli tool to manage GSLTs (Game Server License Tokens) for
SRCDS based gameservers like CounterStrike:GlobalOffensive and Team Fortress 2.
It uses the [Steam API](https://partner.steamgames.com/doc/webapi)
and needs an API token that is allowed to generate GSLTs.

Given an API Token, an APPID and a MEMO this tool:

- creates a new token if none exists with the given APPID and MEMO combination
- regenerates the token if one exists with the given APPID and MEMO combination
- prints the (now valid) token to the standard output

Errors are printed to standard error output.
The return code is used as follows:

## Return Codes

| Code | Meaning |
|------|---------|
|    0 | everything went all right. |
|    1 | could not find the `GSLTCTRL_TOKEN` environment variable. |
|    2 | invalid commandline arguments |
|    3 | The API responded with an error code |
|    4 | Error while parsing JSON response |
|    5 | The API responded with an invalid body |
|    6 | sending a request to the API was unsucessfull |
|    7 | could not format a JSON message |

# Download

You can download the latest binary for your os under releases, or you can build it with the instructions below.

# API used by GSLTCTRL-RS

This software only uses the
[IGameServersService](https://partner.steamgames.com/doc/webapi/IGameServersService)
API.

# Building

This programm is written in rust and requires cargo to build.

To build it, simply run
```
cargo build --release
```
This will download the required dependencies automatically.
The finished binary will be in `target/release/gsltctrl` or on windows `target/release/gsltctrl.exe`.

# Usage

First you have to specifiy your API Token.
This is done via a environment variable called `GSLTCTRL_TOKEN`.
The `APPID` and the `MEMO` are specified on the commandline.

```
GSLTCTRL_TOKEN="0123456789ABCDEF0123456789ABCDEF" gsltctrl APPID MEMO
```

If your memo should have spaces or other funky things in it, put them
in double quotes, e.g.:

```
GSLTCTRL_TOKEN="0123456789ABCDEF0123456789ABCDEF" gsltctrl 730 "My cool CSGO server"
```

with `--help` you can also get an overview of the available options.
