# Solana token info CLI client

Simple CLI client to get information about Solana tokens.

Supports fetching token metadata from Token-2022 and Metaplex Token Metadata.

## Basic usage

```bash
Usage: solana_token_info [OPTIONS] --address <ADDRESS>

Options:
  -d, --debug-level...       Turn debugging information on
  -a, --address <ADDRESS>    Solana token address
  -e, --endpoint <ENDPOINT>  Solana network endpoint [default: mainnet] [possible values: mainnet, devnet, testnet]
  -h, --help                 Print help
  -V, --version              Print version
```

## Sample output

With token metadata from Token-2022:

```bash
cargo run -- -a DaoQK21HaBxJ4t8s8r5HWtF6xBKVqdrKDeGzH8QjwJcp  -e devnet
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/solana_token_info -a DaoQK21HaBxJ4t8s8r5HWtF6xBKVqdrKDeGzH8QjwJcp -e devnet`
Token info:
   * Name: thenes
   * Symbol: THENES
   * Supply 42
   * URI: http://thenes.xyz
   * IPv4 records: 1
   * IPv6 records: 0
```

With token metadata from Metaplex Token Metadata:

```bash
cargo run -- -a Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB  -e mainnet
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
cargo run -- -dd -a Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB  -e mainnet
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/solana_token_info -dd -a Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB -e mainnet`
[*] Setting log level to: Info
[*] The address is: Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB
[*] Endpoint: mainnet
[*] Requesting data from: mainnet (https://api.mainnet-beta.solana.com)
[*] Pubkey parsed: Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB
[*] Token metadata not found in SPL token account, trying to fetch from metadata account
[*] Token metadata found in metadata account
[*] Token metadata: TokenMetadata { name: "USDT", symbol: "USDT", uri: "" }
[*] Token supply: 1889937981.94161
[*] Token metadata URI is empty, trying to fetch from tokenlist
[*] Token website from tokenlist: "https://tether.to/"
[*] Fetching DNS records for website: https://tether.to/
[*] Host: "tether.to"
[*] Answer: ipv4: 3 ipv6: 3
Token info:
   * Name: USDT
   * Symbol: USDT
   * Supply 1889937981.94161
   * URI: https://tether.to/
   * IPv4 records: 3
   * IPv6 records: 3
```
