# Summary
Experimental content-addressable file storage

Uses streaming endpoints in gRPC to handle upload/download and blake3 as the hash for file identity. 

Not intended for production use, just for fun.

# Example
All the examples assume you've run `cargo build --release`
## Server
Running the server via cargo:

`RUST_LOG=info ./target/release/hexagonal-server`

## Client
The example client takes a file from STDIN, uploads it to the server and then re-downloads it again, piping it to STDOUT as an example of using both endpoints provided. 

`cat some_file | RUST_LOG=info ./target/release/hexagonal-client > some_output_file`
