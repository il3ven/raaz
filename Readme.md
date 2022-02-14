## Raaz

A practice project for learning about [Rust](https://www.rust-lang.org/) and [near protocol](https://www.near.org/).

Answer a puzzle. Each guess requires you to deposit min 5 yoctoNear. If you're wrong, the deposit is lost. If you're right then everyone else's deposit is yours.

https://user-images.githubusercontent.com/4337699/153909484-6588f796-cb92-48d4-addd-46039ba03359.mov

## Run the code

### Prequisites
- Rust
- Node.js
- Near CLI
  ```
  npm install -g near-cli
  ```

### Contract

- Change into the `contracts` directory
  ```
  cd contracts
  ```

- Build the contract
  ```
  ./build.sh
  ```

- Deploy the contract
  ```sh
  near create-account <your-smart-contract-name> --masterAccount <your-near-account>
  ```

  Deploy and run the init function in a batch transaction

  ```sh
  near deploy <your-smart-contract-name> --wasmFile res/raaz.wasm --initFunction new --initArgs '{"owner_id": "raaz.shockline.testnet", "question": "What is the capital of Paris?", "solution": "1670f2e42fefa5044d59a65349e47c566009488fc57d7b4376dd5787b59e3c57"}'
  ```

### Client

### Development server

  ```
  npm install
  ```

  ```sh
  npm run start
  ```

## Technology Stack

The project has been built using Rust, Near SDK, React and Parcel. The project also includes test for the smart contract.

The code for the smart contract is simple and self-contained.

## Further Improvements

There is a vulnerability in the smart contract that allows people to guess the solution to the puzzle without a deposit. I realized this after developing the project. Due to limited time, I couldn't fix the vulnerability.

This project can be further improved but I am limited by time. I have spent 3 days on the project. I refuse to spend more time. I have learned Rust and Near from scratch in these 3 days.
