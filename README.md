# cw-plus-tests

Repo for simple example of cw-plus-boot implementations.
They showcase how simple it is to interact with boot contracts using a few libraries and easy to use interfaces

## Usage

Don't forget to create a `.env` file with JUNO testnet mnemonics (using `.env.example`)

Run examples to see the magic of boot happening : 

```bash
RUST_LOG=info cargo run --example cw1-subkeys-chain
```

Or for mock tests

```bash
RUST_LOG=info cargo run --example cw20-base-mock
```
