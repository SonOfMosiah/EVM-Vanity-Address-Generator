# EVM Vanity Address Generator

> A Rust program for finding salts that create gas-efficient Ethereum addresses via CREATE2.

Provide three arguments: a factory address (or contract that will call CREATE2), a caller address (for factory addresses that require it as a protection against frontrunning), and the keccak-256 hash of the bytecode of the contract that the factory will deploy.

```sh
$ git clone https://github.com/ammonwerner1/evm-vanity-address-generator
$ cd evm-vanity-address-generator
$ export FACTORY="<Create2Deployer.sol ADDRESS>"
$ export CALLER="<YOUR_ROPSTEN_ADDRESS_OF_CHOICE_GOES_HERE>"
$ export INIT_CODE_HASH="<HASH_OF_YOUR_CONTRACT_BYTECODE_GOES_HERE>"
$ cargo run --release $FACTORY $CALLER $BYTECODE_HASH
```

For each efficient address found, the salt, resultant addresses, and value _(i.e. approximate rarity)_ will be written to `efficient_addresses.txt`. Verify that one of the salts actually results in the intended address before getting in too deep - ideally, the CREATE2 factory will have a view method for checking what address you'll get for submitting a particular salt. Be sure not to change the factory address or the init code without first removing any existing data to prevent the two salt types from becoming commingled. There's also a _very_ simple monitoring tool available if you run `$python3 analysis.py` in another tab.

There is also an experimental OpenCL feature that can be used to search for addresses using a GPU. To give it a try, include a fourth parameter specifying the device ID to use, and optionally a fifth and sixth parameter to filter returned results by a threshold based on leading zero bytes and total zero bytes, respectively. By way of example, to perform the same search as above, but using OpenCL device 2 and only returning results that create addresses with at least four leading zeroes or six total zeroes, use `$ cargo run --release $FACTORY $CALLER $BYTECODE_HASH 2 4 6` (you'll also probably want to try tweaking the `WORK_SIZE` parameter in `src/lib.rs`).

PRs welcome!
