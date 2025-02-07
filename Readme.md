[![MIT license](https://img.shields.io/github/license/RCasatta/blocks_iterator)](https://github.com/RCasatta/blocks_iterator/blob/master/LICENSE)
[![Crates](https://img.shields.io/crates/v/blocks_iterator.svg)](https://crates.io/crates/blocks_iterator)

# Blocks iterator

Iterates over Bitcoin blocks, decoding data inside Bitcoin Core's blocks directory.

Features:
* Blocks are returned in height order, it avoids following reorgs (see `max_reorg` parameter)
* Blocks come with [metadata](https://docs.rs/blocks_iterator/latest/blocks_iterator/struct.BlockExtra.html) like all block's previous outputs, it allows computing transactions fee or [verifying](examples/verify.rs) scripts in the blockchain.

# Pipes

Other than inside Rust programs, ordered blocks with prevouts could be iterated using Unix pipes.

```
$ cargo build --release 
$ cargo build --release --examples
$ ./target/release/blocks_iterator --blocks-dir /Volumes/Transcend/bitcoin-testnet/testnet3/blocks --network testnet --max-reorg 40 | ./target/release/examples/most_output_pipe | ./target/release/examples/heaviest_pipe >/dev/null
...
[2021-10-21T10:10:24Z INFO  most_output_pipe] most_output tx is d28305817238ee92e5d9ac0d81c3bf5ecf7399528e6d87226d726e4070c7e665 with #outputs: 30001
[2021-10-21T10:10:24Z INFO  heaviest_pipe] heaviest tx is 73e64e38faea386c88a578fd1919bcdba3d0b3af7b6302bf6ee1b423dc4e4333 with weight: 3999608
```

## Memory requirements and performance

Running [iterate](examples/iterate.rs) example on threadripper 1950X, Testnet @ 2090k, Mainnet @ 705k. Spinning disk.

| Network | `--skip--prevout` | `--max-reorg` | `utxo-db` | Memory | Time    |
|---------|-------------------|---------------|----------:|-------:|--------:|
| Mainnet | true              |           6   | no        |   33MB |  1h:00m |
| Mainnet | false             |           6   | no        |  5.3GB |  1h:29m |
| Mainnet | false             |           6   | 1 run     |  201MB |  9h:42m |
| Mainnet | false             |           6   | 2 run     |  113MB |  1h:05m |
| Testnet | true              |           40  | no        |  123MB |  3m:03s |
| Testnet | false             |           40  | no        |  1.4GB |  9m:07s |
| Testnet | false             |           40  | 1 run     |  247MB | 16m:12s |
| Testnet | false             |           40  | 2 run     |  221MB |  8m:32s |

## Examples

Run examples with:

```
cargo run --release --example iterate
```

* [iterate](examples/iterate.rs) iterate over blocks and print block fee
* [heaviest](examples/heaviest_pipe) find the transaction with greatest weight
* [most_output](examples/most_output_pipe) find the transaction with most output
* [verify](examples/verify.rs) verify transactions

## Similar projects

* [bitcoin-iterate](https://github.com/rustyrussell/bitcoin-iterate) this project inspired blocks_iterator, the differences are:
  * bitcoin-iterate is C-based
  * bitcoin-iterate it's more suited for shell piping, while blocks_itearator is intended to use as a rust library
  * bitcoin-iterate doesn't give previous outputs information
  * bitcoin-iterate is making two passage over `blocks*.dat` while blocks_iterator is doing one pass keeping out-order-blocks in memory (the latter is faster at the expense of higher memory usage)
* [rust-bitcoin-indexer](https://github.com/dpc/rust-bitcoin-indexer) this project requires longer setup times (about 9 hours indexing) and a postgre database, once the indexing is finished it allows fast queries on relational database.
* [electrs](https://github.com/romanz/electrs) specifically intended for the Electrum protocol
