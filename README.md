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
  repl         Interact with Ditto via REPL interface
  collections  Interact with Ditto's collections for your App ID
  configure    Create or show dto's configuration file
  utils        Interface with utilities and presence information
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>  Sets a custom config file
  -v, --verbose...     Turn debugging information on
  -h, --help           Print help
  -V, --version        Print version
```

TODO: Update this ^^

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
