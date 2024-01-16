Inspired by [ethereum chains](https://github.com/ethereum-lists/chains).

A simple tool for querying RPC, Token info of the Ethereum chains.

## Install

```sh
cargo install eth-chains-cli
```

## Usage

- Get chain info by id

```sh
$ eth-chains-cli by-id --id 1

╭-----------------+-------------------------------------------------╮
| ITEMS           | VALUE                                           |
+===================================================================+
| CHAIN_NAME      | Ethereum Mainnet                                |
|-----------------+-------------------------------------------------|
| CHAIN_ID        | 1                                               |
|-----------------+-------------------------------------------------|
| NATIVE_CURRENCY | Ether                                           |
|-----------------+-------------------------------------------------|
| SYMBOL          | ETH                                             |
|-----------------+-------------------------------------------------|
| DECIMALS        | 18                                              |
|-----------------+-------------------------------------------------|
| NETWORK         | 1                                               |
|-----------------+-------------------------------------------------|
| INFO            | https://ethereum.org                            |
|-----------------+-------------------------------------------------|
| RPC             | https://mainnet.infura.io/v3/${INFURA_API_KEY}  |
|                 | wss://mainnet.infura.io/ws/v3/${INFURA_API_KEY} |
|                 | https://api.mycryptoapi.com/eth                 |
|                 | https://cloudflare-eth.com                      |
|                 | https://ethereum.publicnode.com                 |
|                 | wss://ethereum.publicnode.com                   |
|                 | https://mainnet.gateway.tenderly.co             |
|                 | wss://mainnet.gateway.tenderly.co               |
|                 | https://rpc.blocknative.com/boost               |
|                 | https://rpc.flashbots.net                       |
|                 | https://rpc.flashbots.net/fast                  |
|                 | https://rpc.mevblocker.io                       |
|                 | https://rpc.mevblocker.io/fast                  |
|                 | https://rpc.mevblocker.io/noreverts             |
|                 | https://rpc.mevblocker.io/fullprivacy           |
|-----------------+-------------------------------------------------|
| FAUCETS         | None                                            |
|-----------------+-------------------------------------------------|
| EXPLORERS       | etherscan https://etherscan.io                  |
|                 | blockscout https://eth.blockscout.com           |
|                 | dexguru https://ethereum.dex.guru               |
|-----------------+-------------------------------------------------|
| FEATURES        | EIP155                                          |
|                 | EIP1559                                         |
╰-----------------+-------------------------------------------------╯
```

- Get chain by name

```sh
$ eth-chains-cli by-name -n "Ethereum"

╭------------------------+----------------------╮
| Candidate Chain's Name | Candidate Chain's ID |
+===============================================+
| Ethereum Mainnet       | 1                    |
|------------------------+----------------------|
| Ethereum Classic       | 61                   |
|------------------------+----------------------|
| ethereum Fair          | 513100               |
╰------------------------+----------------------╯
```

- Get all chains list

```sh
$ eth-chains-cli list

╭──────────────────────┬──────────┬───────────────────────┬────────┬──────────╮
│ CHAIN_NAME          ┆ CHAIN_ID ┆ NATIVE_CURRENCY       ┆ SYMBOL ┆ DECIMALS│
╞══════════════════════╪══════════╪═══════════════════════╪════════╪══════════╡
│ Ethereum Mainnet    ┆ 1        ┆ Ether                 ┆ ETH    ┆ 18      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ Expanse Network     ┆ 2        ┆ Expanse Network Ether ┆ EXP    ┆ 18      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ Ropsten             ┆ 3        ┆ Ropsten Ether         ┆ ETH    ┆ 18      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ Rinkeby             ┆ 4        ┆ Rinkeby Ether         ┆ ETH    ┆ 18      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ Goerli              ┆ 5        ┆ Goerli Ether          ┆ ETH    ┆ 18      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ Kotti Testnet       ┆ 6        ┆ Kotti Ether           ┆ KOT    ┆ 18      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ ThaiChain           ┆ 7        ┆ ThaiChain Ether       ┆ TCH    ┆ 18      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ Ubiq                ┆ 8        ┆ Ubiq Ether            ┆ UBQ    ┆ 18      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ Ubiq Network Testnet┆ 9        ┆ Ubiq Testnet Ether    ┆ TUBQ   ┆ 18      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ OP Mainnet          ┆ 10       ┆ Ether                 ┆ ETH    ┆ 18      │
╰──────────────────────┴──────────┴───────────────────────┴────────┴──────────╯
.....
```
