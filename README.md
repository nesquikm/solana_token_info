## Solana token info CLI client

Simple CLI client to get information about Solana tokens.

### Basic usage

```bash
Usage: solana_token_info [OPTIONS] --address <ADDRESS>

Options:
  -d, --debug-level...       Turn debugging information on
  -a, --address <ADDRESS>    Solana token address
  -e, --endpoint <ENDPOINT>  Solana network endpoint [default: mainnet] [possible values: mainnet, devnet, testnet]
  -h, --help                 Print help
  -V, --version              Print version
```

### Sample output

```bash
➜  solana_token_info cargo run -- -a Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB  -e mainnet
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/solana_token_info -a Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB -e mainnet`
Token info:
   * Name: USDT
   * Symbol: USDT
   * Supply 1889937984.92189
   * URI: https://tether.to/
   * IPv4 records: 3
   * IPv6 records: 3
```

You can add debug level to see more information:

```bash
argo run -- -dd -a Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB  -e mainnet
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/solana_token_info -dd -a Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB -e mainnet`
[*] Setting log level to: Info
[*] The address is: Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB
[*] Endpoint: mainnet
[*] Requesting data from: mainnet (https://api.mainnet-beta.solana.com)
[*] Pubkey parsed: Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB
[*] Token metadata: TokenMetadata { name: "USDT", symbol: "USDT", uri: "" }
[*] Token supply: 1889937984.92189
[*] Token metadata URI is empty, trying to fetch from tokenlist
[*] Token website from tokenlist: "https://tether.to/"
[*] Fetching DNS records for website: https://tether.to/
[*] Host: "tether.to"
[*] Answer: ipv4: 3 ipv6: 3
Token info:
   * Name: USDT
   * Symbol: USDT
   * Supply 1889937984.92189
   * URI: https://tether.to/
   * IPv4 records: 3
   * IPv6 records: 3
```
