# Postgresql Heartbeat

> **Note**
> 
> This was written by somebody who does not know Rust! Please be patient with any bugs.

Sends a heartbeat to a URL if it can connect to a Postgres DB.

## Setup
### Docker Image
> *coming soon™*

### Custom Build
1. Run `cargo build --release`
2. The executable can be found in `target/release/postgres-heartbeat`
3. Set the following environment variables:
    * `HEART_SEC` How many seconds should be between request attempts
    * `HEART_URL` The URL to send the heartbeat to
    * `HEART_CONN` The connection URL of the postgresql database
4. Run the `postgres-heartbeat` executable
