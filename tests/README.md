## Deposit Demo

This test file in [`deposit.ts`](./deposit.ts) demonstrates how to use the transaction
relayer to relay a deposit transaction.

### Prerequisites

1. You can use Bun or node.js to run the test file.
2. Tangle Testnet running locally.
3. This Relayer running locally at `http://localhost:3000`.

### Running Tangle Testnet
1. Compile the tangle testnet node using the following command:
```bash
cargo b -rp tangle --features txpool,testnet,manual-seal
```
2. Run the tangle testnet node using the following command:
```bash
./target/release/tangle --tmp --dev --validator -linfo --alice --rpc-cors all --rpc-methods=unsafe --rpc-external --rpc-port 9944 -levm=debug -lg
adget=trace --sealing manual
```

### Running the Relayer
1. Clone this repository and then run the following commands:
```bash
cargo build
```
2. Install the Tangle CLI if you haven't already:
```bash
cargo install cargo-tangle --force
```
3. Insert the following key into the keystore:
```bash
mkdir -p target/keystore
cargo tangle blueprint keygen -k ecdsa -p target/keystore -s cb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854
```
4. Run the relayer using the following command:
```bash
RUST_LOG=gadget=debug,tower_http=debug,txrelayer_blueprint=debug ./target/debug/txrelayer-blueprint run --protocol tangle --blueprint-id 0 --service-id 0 --http-rpc-url http://localhost:9944 --ws-rpc-url ws://localhost:9944 --chain local_testnet --keystore-uri ./target/keystore
```

### Running the Deposit Demo

Go to the `tests` directory and then:

1. Install the dependencies using the following command:
```bash
npm install
```
or if you are using bun:
```bash
bun install
```
2. Run the deposit demo using the following command:
```bash
bun deposit.ts
```

### Expected Output
```log
Request Body: {
  "from": "0x25451A4de12dcCc2D166922fA938E900fCc4ED24",
  "to": "0x0000000000000000000000000000000000000822",
  "value": "0x0",
  "data": "0xb3c11395000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008ac7230489e800000000000000000000000000000000000000000000000000000000000000000000",
  "gaslimit": 600000,
  "deadline": "0x6792767d",
  "v": 28,
  "r": "0x108088dbcffe4c0db845424ae41a2bc8f730881e5843b7e4f9040bf405ab660c",
  "s": "0x66731ce89003c237074754fada695d5d2b33054d094de1bd5f14136a413e0758"
}
Response: {
  "status": "failure",
  "error": "VM Exception while processing transaction: revert Dispatched call failed with error: Module(ModuleError { index: 45, error: [50, 0, 0, 0], message: Some(\"DepositExceedsCapForAsset\") })",
  "details": "0x08c379a000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000086446973706174636865642063616c6c206661696c65642077697468206572726f723a204d6f64756c65284d6f64756c654572726f72207b20696e6465783a2034352c206572726f723a205b35302c20302c20302c20305d2c206d6573736167653a20536f6d6528224465706f73697445786365656473436170466f7241737365742229207d290000000000000000000000000000000000000000000000000000"
}
Error: VM Exception while processing transaction: revert Dispatched call failed with error: Module(ModuleError { index: 45, error: [50, 0, 0, 0], message: Some("DepositExceedsCapForAsset") })
Details: 0x08c379a000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000086446973706174636865642063616c6c206661696c65642077697468206572726f723a204d6f64756c65284d6f64756c654572726f72207b20696e6465783a2034352c206572726f723a205b35302c20302c20302c20305d2c206d6573736167653a20536f6d6528224465706f73697445786365656473436170466f7241737365742229207d290000000000000000000000000000000000000000000000000000
```

Even though this transaction fails, it demonstrates how to use the transaction relayer to relay transactions.
