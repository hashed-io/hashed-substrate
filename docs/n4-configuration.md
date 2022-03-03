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
  - [Configuring systemd and pm2 services](#configuring-systemd-and-pm2-services)
    - [Polkadot](#polkadot-1)
    - [Liberland projects](#liberland-projects-1)
      - [Liberland frontend](#liberland-frontend-1)
      - [Liberland backend](#liberland-backend-1)
  - [Nginx configuration](#nginx-configuration)
  - [Appendix](#appendix)
    - [List of ports per project](#list-of-ports-per-project)
      - [Kusama (using the default ports)](#kusama-using-the-default-ports)
      - [Liberland node (increment by 1)](#liberland-node-increment-by-1)
      - [Hashed (increment by 2)](#hashed-increment-by-2)
      - [Liberland frontend: 8080](#liberland-frontend-8080)
      - [Liberland backend: 3000](#liberland-backend-3000)

## Prerequisites

- Install nginx:
```
sudo apt update
sudo apt install nginx
```
- Update rust (just in case)
 
 ```
 rustup update
 ```

- Install node version manager, node and yarn:

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
# Terminal restart needed
# Install a LTS node version (v14.19.0 aka Fermium)
nvm install lts/fermium

npm install --global yarn
# Node daemon manager, it'll come handy when deploying services
npm install pm2 -g

```

- The liberland frontend project uses some dependencies that strictly require some version of python2 or lower (setting up an alias and exporting the python3 path were tried without success):
```
sudo apt install python2
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
# Assuming you're at ~/liberland directory
cd liberland_node
cargo build --release
```

#### Liberland frontend

The frontend consists on a yarn/npm project, so it is needed to install the dependencies before building the project:

```bash
cd ~/liberland/liberland_frontend
# There's some minor versioning issues, so the --force flag will
# allow the installation 
npm install
yarn build
```

#### Liberland backend
The backend project doesn't need any additonal configuration besides installing its dependencies:

```bash
cd ~/liberland/liberland_backend
npm install
```

### Hashed chain
A hashed chain project is already deployed on the n4 machine; only an update on the main branch was performed:

```bash 
cd ~/hashed
# Check for updates:
git fetch origin main
git pull origin main
# Build the project (Unfortunately, the main branch gets some compilation
# errors at the current moment)
cargo build --release
```

## Configuring systemd and pm2 services

### Polkadot

### Liberland projects
Because both frontend and backend are node-based projects, it was decided to use pm2 to daemonize and manage them.


In order to create the pm2 service on systemd, the easiest way is typing the following commands:

```bash

pm2 startup systemd
# The output of the previous command should print something similar to the next command:
sudo env PATH=$PATH:/home/max/.nvm/versions/node/v14.19.0/bin /home/max/.nvm/versions/node/v14.19.0/lib/node_modules/pm2/bin/pm2 startup systemd -u max --hp /home/max

reboot
# After those two commands, it is possible to consult the service status:
systemctl status pm2-max

```

#### Liberland frontend

In oder to daemonize the built, static frontend project, the following command was typed:

```bash
# 8080 being the port
pm2 serve /home/max/liberland/liberland_frontend/build/ 8080 -n frontend

# save the current process list
pm2 save
```
#### Liberland backend

The backend project can't be directly built, so the commands may differ from the previous project:

```
pm2 start /home/max/liberland/liberland_backend/app.js -n backend

# save the current process list
pm2 save
```

Note: It isn't needed to specify or change the port because the default port is the desired one (3000 for backend)

Useful conmmands

```bash
./target/release/polkadot --chain=kusama
# Tell to max its deployed on dev mode
./target/release/liberland_node --dev --tmp

# see telos for references

# check if nginx conf is ok
sudo nginx -t

pm2 list
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