# wgx

wireguard tool to manage / generate configuration. Maintain one yaml configuration file to quickly build wireguard network.

## Usage

![wgx usage demo](./res/demo.svg)
_No demo visible here? View it on [asciinema](https://asciinema.org/a/515270)._

```bash
wgx --h

USAGE:
    wgx [OPTIONS]

OPTIONS:
    -a, --all                Include all nodes
    -c, --config <CONFIG>    Config file [default: ./config.yml]
    -f, --format <FORMAT>    Output format [default: conf]
    -h, --help               Print help information
    -n, --node <NODE>        Node to export [default: ]
    -V, --version            Print version information
```

more information please checkout [config.example.yml](./config.example.yml)

[usage-demo]: https://asciinema.org/a/515270
