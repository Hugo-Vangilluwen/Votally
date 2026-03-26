# Votally

Votally is a voting application that runs over the local network.

To host a vote, first [run the Votally server](#run-the-server) and specify the voting parameters. Users can then connect to the server using [the Votally cli](#run-the-client) and submit their votes.

This is a work in progress.

Do not use for major votes.

`libvotally` contains the core voting logic. `votally-cli` is thin wrapper exposing a CLI.

## Build
The followign commands build the Votally CLI. The generated binary can be found at `target/release/votally-cli`.
```bash
cargo build --release
```

## Run the server
```bash
votally-cli --server options...
```
where `options` is a list of the available choices in the vote.
The `--voting-system` option can also be specified to change the voting system.

The server's IP address will then be displayed.
Users should then use the client to connect to the server.
Once all users are connected, press enter to begin voting process.
Then, once all voting are cast, press enter again to count the votes.


## Run the client
Run the client on the same local network as the server using
```bash
votally-cli
```

Enter the server's IP. You will then be presented with the voting system in use and the list of available options. Once voting starts, submit your vote as indicated and wait for voting to end; you will then be shown the winning option.
