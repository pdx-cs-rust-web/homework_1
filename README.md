# Question Server
Casey Bailey and Bart Massey 2023

This Rust workspace contains an Axum HTTP server, a command-line client, and shared question types. Data is stored in memory while the server is running. See [API.md](API.md) for the HTTP interface.

## Run

Install a current Rust toolchain, then start the server from the `backend` repository root:

```sh
cd backend
cp env.example .env
cargo run
```

In another terminal, run the client:

```sh
cargo run -p client
```

The server listens at `http://127.0.0.1:8088`. The client is starter code and exits without making a request until its request logic is implemented.
