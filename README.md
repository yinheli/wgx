# wgx

wireguard tool to manage / generate configuration. Maintain one yaml configuration file to quickly build wireguard network.

## Usage

[![asciicast](https://asciinema.org/a/NIP9rpjjlWVIMM6hUl4Q8U4ap.svg)](https://asciinema.org/a/NIP9rpjjlWVIMM6hUl4Q8U4ap)

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
