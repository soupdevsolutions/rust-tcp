# rust-tcp

This is an example on how to build a custom messaging protocol over TCP using Rust and Tokio.  
You can find the tutorial walking you through the code on the [soup.dev blog](https://www.soup.dev/post/building-a-custom-protocol-over-tcp-with-rust-and-tokio) or on [Medium](https://medium.com/@mirceaoprea/building-a-custom-protocol-over-tcp-with-rust-and-tokio-ab264c0d3e07).


## Running the project

The project includes 2 binaries - a server and a client - that you can run to check out how the protocol works.  
After cloning the project, start the server in one terminal:

```bash
cargo run --bin server
```

and the client in another one:

```bash
cargo run --bin client
```

You can make changes to the client binary in `src/bins/client.rs` to send different messages to the server.
