# dto

`dto` is a simple CLI for interacting with a local Ditto database.

This is a concept at this point, pre-Alpha. :) Looking for feedback!

## Issues?

Start here: https://github.com/getditto/dto/issues

## Configuration

A valid `app_id` and `license_token` are required (get from https://portal.ditto.live).

Can run `dto configure` to create the template in `~/.config/dto.toml`.

Then drop in those values and you'll connect to the Big Peer, and can make connections to Small Peers to get data into a local (to where the command is run) database.

Example contents of the config file:
```
app_id = "XXXXXXXX-XXXX-XXXX-XXXX-9c2475077258"
license_token = "XXXXXXXX-XXXX-XXXX-XXXX-94450106cc9d"
```

## Running

`dto --help`

```
Interact with a Ditto database.

Usage: dto [OPTIONS] [COMMAND]

Commands:
  collections  View Ditto's local collections for your App ID
  configure    Create or show dto's configuration file
  execute      Execute a single synchronous query for a given configured APP ID
  observe      Observe data on a specific query for a given configured APP ID
  presence     Observe local and remote peer metadata
  utils        Ditto database utilities and information
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>  Sets a custom config file
  -v, --verbose...     Turn debugging information on
  -h, --help           Print help
  -V, --version        Print version
```

### Execute
Run a simple synchronous query against the local database.

```
❯ ./target/debug/dto execute -q "SELECT * FROM stuff"
[{"_id":"655e8ffb00abc84c005c79cc","hello":"world"},{"_id":"655e90af00e2d012007e3c13","hello":"world"},{"_id":"656690d700e648800088d752","hello":"world"},{"_id":"65677fc800fb63c700cf4d4c","hello":"world"},{"_id":"6568d435005ef460005a5372","hello":"world"},{"_id":"6568d4e200b94b0600b0b3df","hello":"world"},{"_id":"6568dd040060b9570064c3e6","hello":"world"},{"_id":"6568e30300dbe14700fdc899","hello":"world"}]
```

### Observe
Like the `execute` command, `observe` will execute a query, but following a
subscription, and then will register an observer and report with updates as they
are synced from a peer.

### Presence
Presence information is useful in debugging and testing things in Ditto.

Local peer information?
```
❯ ./target/debug/dto presence -s local
{"address":{"pubkey":[2,135,2,130,67,2,43,74,26,9,184,153,93,86,48,246,247,104,51,163,208,114,253,36,74,190,185,9,217,77,118,150,149,58,51,110,102,98],"siteId":18021064730814713115},"connections":[],"deviceName":"dto-cli","dittoSdkVersion":"4.5.0","isCompatible":true,"isConnectedToDittoCloud":false,"os":"macOS","peerKey":[2,135,2,130,67,2,43,74,26,9,184,153,93,86,48,246,247,104,51,163,208,114,253,36,74,190,185,9,217,77,118,150,149,58,51,110,102,98],"queryOverlapGroup":0}
```

Or how bout listening for remote peer activity and changes in the mesh?
```
❯ ./target/debug/dto presence -s remote
mesh change observed (connected to BP? false.)
remote peers:
mesh change observed (connected to BP? true.)
remote peers:
  peer: ditto, type: AccessPoint, sdk:4.4.5
mesh change observed (connected to BP? true.)
remote peers:
```

### Collections

```
❯ dto collections -l
collections here!
Ok(
    [
        "stuff",
    ],
)
```

## Dev and Test

For development, write code, write test - in any order as long as it is `&&`.

To run the tests: `cargo test`

To run a specific test: `cargo test test_observe_valid_query`

To contribute - create a branch, add/fix, test, submit a PR.

## License

MIT - Copyright © 2023 <DittoLive>
