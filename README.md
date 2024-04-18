<h1 align="center">
  Nifty CLI
</h1>
<p align="center">
  <img width="400" alt="Nifty CLI" src="https://github.com/nifty-oss/cli/assets/729235/b078b393-350b-4340-80b0-f19b6275d6fc" />
</p>
<p align="center">
  A CLI for interacting with the Nifty Asset program.
</p>

<p align="center">
  <a href="https://github.com/nifty-oss/asset/actions/workflows/main.yml"><img src="https://img.shields.io/github/actions/workflow/status/nifty-oss/cli/main.yml?logo=GitHub" /></a>
  <a href="https://github.com/nifty-oss/cli/releases/latest"><img src="https://img.shields.io/github/v/release/nifty-oss/cli" /></a>
</p>

## Installation

Via install script:

```bash
bash <(curl -sSf https://raw.githubusercontent.com/nifty-oss/cli/main/scripts/install.sh)
```

From source:

```bash
cargo install --path .
```

## Commands

### Burn

Burns an asset, closing the account reclaiming all rent.

```
Usage: nifty burn [OPTIONS] <ASSET> [RECIPIENT]

Arguments:
  <ASSET>      The asset to burn
  [RECIPIENT]  The recipient to receive reclaimed rent. Defaults to the signer

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

Examples: 

No recipient specified, so reclaimed rent goes to the signing keypair:

```bash
nifty burn 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

Recipient specified and receives reclaimed rent:

```bash
nifty burn 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP 9Z3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

### Mint 

Create an asset with extension data

```
Usage: nifty mint [OPTIONS] <ASSET_FILE_PATH>

Arguments:
  <ASSET_FILE_PATH>  

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

### Mint-Batch

Create a batch of assets with extension data

```
Usage: nifty mint-batch [OPTIONS] <ASSET_FILES_DIR>

Arguments:
  <ASSET_FILES_DIR>

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

### Create

Creates a new asset.

```
Usage: nifty create [OPTIONS] --name <NAME>

Options:
  -k, --keypair-path <KEYPAIR_PATH>
          Path to the keypair file
  -n, --name <NAME>
          The name of the asset
  -a, --asset-keypair-path <ASSET_KEYPAIR_PATH>
          Path to the mint keypair file
  -r, --rpc-url <RPC_URL>
          RPC URL for the Solana cluster
      --immutable
          Create the asset as immutable
  -o, --owner <OWNER>
          Owner of the created asset, defaults to authority pubkey
  -h, --help
          Print help
  ```

Examples:

Create a mutable asset:

```bash
nifty create --name "My Asset"
```

Create an immutable asset:

```bash
nifty create --name "My Immutable Asset" --immutable
```

Create an asset with a specific owner:

```bash
nifty create --name "My Asset" --owner 9Z3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

Create an asset from an existing keypair file:

```bash
nifty create --name "My Asset" --asset-keypair-path /path/to/asset-keypair.json
```

### Decode

Decodes an asset into a human readable format.

```
Usage: nifty decode [OPTIONS] <ASSET>

Arguments:
  <ASSET>

Options:
  -f, --field <FIELD>                The field to decode. If not specified, the entire asset will be decoded
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

Example:

```bash
nifty decode 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

Decode a specific field:

```bash
nifty decode 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP --field state
```

prints only the state information of the asset.

### Approve

Set a delegate on an asset with specific roles

```
Usage: nifty approve [OPTIONS] <ASSET> <DELEGATE>

Arguments:
  <ASSET>     The asset to delegate
  <DELEGATE>  The address to delegate to

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -R, --role <ROLE>                  The role for the delegate to have: "burn", "lock", "transfer". Specify each one separately: --role burn --role lock --role transfer
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help

Example:

```bash
nifty approve 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP 9Z3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP --role burn --role lock
```

### Revoke

Revoke a delegate from an asset

```
Revoke a delegate from an asset

Usage: nifty revoke [OPTIONS] <ASSET>

Arguments:
  <ASSET>  The asset to revoke the delegate from

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -R, --role <ROLE>                  The roles to revoke: "burn", "lock", "transfer". Specify each one separately: --role burn --role lock --role transfer
      --all                          Revoke all roles from the delegate and clear it
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

### Lock

Lock an asset, preventing any actions to be performed on it.

```
Usage: nifty lock [OPTIONS] <ASSET> [DELEGATE_KEYPAIR_PATH]

Arguments:
  <ASSET>                  The asset to lock
  [DELEGATE_KEYPAIR_PATH]  Path to the delegate keypair file. Defaults to the signer

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

Example:

```bash
nifty lock 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

The keypair used must be the delegate of the asset.


### Unlock

Unlock an asset, allowing actions to be performed on it.

```
Usage: nifty unlock [OPTIONS] <ASSET> [DELEGATE_KEYPAIR_PATH]

Arguments:
  <ASSET>                  The asset to unlock
  [DELEGATE_KEYPAIR_PATH]  Path to the delegate keypair file. Defaults to the signer

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

Example:

```bash
nifty unlock 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

The keypair used must be the delegate of the asset.

### Transfer

Transfers an asset to a new owner.

```
Usage: nifty transfer [OPTIONS] <ASSET> <RECIPIENT>

Arguments:
  <ASSET>      The asset to transfer
  <RECIPIENT>  The recipient of the asset

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
  ```

Example:

  ```bash
  nifty transfer 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP 9Z3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
  ```

## License

Copyright (c) 2024 nifty-oss maintainers

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
