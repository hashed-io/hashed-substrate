# N4 server configuration for multiple susbtrate nodes

- [N4 server configuration for multiple susbtrate nodes](#n4-server-configuration-for-multiple-susbtrate-nodes)
  - [Prerequisites](#prerequisites)
  - [Clone and build git projects](#clone-and-build-git-projects)
    - [Polkadot](#polkadot)
    - [Liberland projects](#liberland-projects)
      - [Liberland node](#liberland-node)
      - [Liberland frontend](#liberland-frontend)
      - [Liberland backend](#liberland-backend)
    - [Hashed chain](#hashed-chain)
  - [Configuring services](#configuring-services)
  - [Nginx configuration](#nginx-configuration)
  - [Appendix](#appendix)
    - [List of ports per project](#list-of-ports-per-project)
      - [Kusama (using the default ports)](#kusama-using-the-default-ports)
      - [Liberland node (increment by 1)](#liberland-node-increment-by-1)
      - [Hashed (increment by 2)](#hashed-increment-by-2)
      - [Liberland frontend: 8080](#liberland-frontend-8080)
      - [Liberland backend: 3000](#liberland-backend-3000)

## Prerequisites
- Update rust (just in case): `rustup update`
- Install node version manager, node and yarn:

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
# Terminal restart needed
# Install the latest LTS (v16.14.0 aka Gallium)
nvm install Gallium

npm install --global yarn
```

## Clone and build git projects

### Polkadot
The official polkadot github project supports both mainnet and kusama runtimes once compiled, so it is only needed to clone the repository and building the latest release branch:

```bash
git clone https://github.com/paritytech/polkadot.git
cd polkadot
# Search the latest release version retrieved from the github repo. 
# It should print something like remotes/origin/release-v*.*.*
git branch -a --contains tags/v0.9.17-rc4
# Clone that remote branch to a local one:
git checkout -t origin/release-v0.9.17
# Then run the set-up script and build 
# (It may print some warnings and/or take some time):
./scripts/init.sh
cargo build --release
```

Note: It is known that apt & rnf package managers host their own polkadot release, allowing a faster installation and service setup. However, it was decided to build the project from a source level in order to provide us more control regarding the updating process in the future, as well as write a more explicit documentation.

### Liberland projects
There are currently 3 liberland-related projects: liberland-node, liberland-frontend and liberland-backend, it is required to download and build them:

```bash
# Return to home:
cd ~
# Make a directory where all the projects will be cloned:
mkdir liberland
cd liberland
# Clone all the github repos:
git clone https://github.com/liberland/liberland_node.git
git clone https://github.com/liberland/liberland_frontend.git
git clone https://github.com/liberland/liberland_backend.git

```
#### Liberland node
The node is built in a similar way to the polkadot project:

```bash
cd liberland_node
# Check for updates:
git fetch origin main
git pull origin main
# Build the project
cargo build --release
```

#### Liberland frontend

#### Liberland backend

### Hashed chain
A hashed chain project is already deployed on the n4 machine; only an update on the main branch was performed:

```bash 
cd ~/hashed

# Unfortunately, the main branch gets some compilation errors 
# at the current moment

```

## Configuring services


Useful conmmands

```bash
./target/release/polkadot --chain=kusama
# Tell to max its deployed on dev mode
./target/release/liberland_node --dev --tmp

# see telos for references

# check if nginx conf is ok
sudo nginx -t
```
## Nginx configuration


## Appendix

### List of ports per project

#### Kusama (using the default ports)

| Service | flag/option | port |
|---------|-------------|------|
| p2p port | `--port` | 30333 |
| WebSocket port | `--ws-port` | 9944 |
| rcp port | `--rpc-port` | 9933 |

#### Liberland node (increment by 1)

| Service | flag/option | port |
|---------|-------------|------|
| p2p port | `--port` |  |
| WebSocket port | `--ws-port` |  |
| rcp port | `--rpc-port` |  |

#### Hashed (increment by 2)

| Service | flag/option | port |
|---------|-------------|------|
| p2p port | `--port` |  |
| WebSocket port | `--ws-port` |  |
| rcp port | `--rpc-port` |  |

#### Liberland frontend: 8080
#### Liberland backend: 3000