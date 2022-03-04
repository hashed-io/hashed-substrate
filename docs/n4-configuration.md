# N4 server configuration for multiple susbtrate nodes

- [N4 server configuration for multiple susbtrate nodes](#n4-server-configuration-for-multiple-susbtrate-nodes)
  - [Prerequisites](#prerequisites)
  - [Clone and build git projects](#clone-and-build-git-projects)
    - [Polkadot](#polkadot)
    - [Liberland projects](#liberland-projects)
      - [Liberland frontend](#liberland-frontend)
      - [Liberland backend](#liberland-backend)
      - [Liberland node](#liberland-node)
    - [Hashed chain](#hashed-chain)
  - [Configuring systemd and pm2 services](#configuring-systemd-and-pm2-services)
    - [Polkadot](#polkadot-1)
    - [Liberland projects](#liberland-projects-1)
      - [Liberland frontend](#liberland-frontend-1)
      - [Liberland backend](#liberland-backend-1)
      - [Liberland node](#liberland-node-1)
      - [Hashed node](#hashed-node)
  - [SSL setup (firewall configuration)](#ssl-setup-firewall-configuration)
  - [Nginx configuration](#nginx-configuration)
  - [Certbot Certificates](#certbot-certificates)
  - [Appendix](#appendix)
    - [List of ports per project](#list-of-ports-per-project)
      - [Kusama (using the default ports)](#kusama-using-the-default-ports)
      - [Liberland node (increment by 1)](#liberland-node-increment-by-1)
      - [Hashed (increment by 2)](#hashed-increment-by-2)
      - [Liberland frontend: 8080](#liberland-frontend-8080)
      - [Liberland backend: 3000](#liberland-backend-3000)
    - [Useful commands](#useful-commands)

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

- Installing certbot is essential:

```
sudo apt install snapd
sudo snap install core; sudo snap refresh core
sudo snap install --classic certbot
sudo ln -s /snap/bin/certbot /usr/bin/certbot
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

It is known that apt & rnf package managers host their own polkadot release, allowing a faster installation and service setup. However, it was decided to build the project from a source level in order to provide us more control regarding the updating process in the future, as well as write a more explicit documentation.

### Liberland projects
There are currently 3 liberland-related projects: liberland-node, liberland-frontend and liberland-backend, it is required to download and build them:

```bash
# Return to home:
cd ~
# Make a directory where all the projects will be cloned:
mkdir liberland
cd liberland
# Clone all the github repos:
git clone https://github.com/liberland/liberland_frontend.git
git clone https://github.com/liberland/liberland_backend.git
git clone https://github.com/liberland/liberland_node.git

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

#### Liberland node
The node is built in a similar way to the polkadot project:

```bash
# Assuming you're at ~/liberland directory
cd liberland_node
cargo build --release
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
In order to set-up a kusama validator as a systemd service, it is needed to create a service configuration file:

```bash
sudo touch /etc/systemd/system/kusama-validator.service
sudo nano /etc/systemd/system/kusama-validator.service
```

The file contents consist in defining which command it will execute:

```vim
[Unit]
Description=Kusama Validator

[Service]
ExecStart=/home/max/polkadot/target/release/polkadot --validator --name "Hashed kusama node" --chain kusama
Restart=always
RestartSec=120

[Install]
WantedBy=multi-user.target
```

It is important to mention the validation service can be enabled only after the chain sincronization process (which may take a while):
```bash
# It is necessary to sync with the kusama chain:

#
systemctl enable kusama-validator.service
```

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

#### Liberland node

Firstly, a chain spec needs to be generated:

```bash
cd ~/liberland/liberland_node
#Generate the chain spec
./target/release/liberland-node build-spec > chain-spec-plain.json

./target/release/liberland-node build-spec --chain chain-spec-plain.json --raw > chain-spec.json
```

Create the service file:

```
sudo touch /etc/systemd/system/liberland-validator.service
sudo nano /etc/systemd/system/liberland-validator.service
```

Service file contents:

```vim
[Unit]
Description=Liberland Validator

[Service]
ExecStart=/home/max/liberland/liberland_node/target/release/liberland-node --chain /home/max/liberland/liberland_node/chain-spec.json --name n1 --validator --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --port 30334 --ws-port 9945 --rpc-port 9934
Restart=always
RestartSec=120

[Install]
WantedBy=multi-user.target
```

Finally, the service can be enabled:
```bash
systemctl enable liberland-validator.service
```

#### Hashed node

For this project, a chain spec and systemd service were already defined, so it will be modified in order to avoid port issues:

```bash
sudo nano /etc/systemd/system/hashed-validator.service
```

```bash
#Added at the end of ExecStart command
--port 30335 --ws-port 9946 --rpc-port 9935
```
Final hashed service configuration:
```bash
[Unit]
Description=Hashed Chaos Validator

[Service]
ExecStart=/home/max/hashed/hashed-substrate/target/release/hashed --base-path /home/max/hashed/hashed-substrate/hashed-chaos-data --chain /home/max/hashed/hashed-substrate/hashed-chaos-spec-raw.json --name n4 --validator --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --bootnodes /ip4/206.221.189.10/tcp/30333/p2p/12D3KooWL7R8De1mPmCj3zA2pMEJXzbDrJVeEJf2SudV21EK9LxU --port 30335 --ws-port 9946 --rpc-port 9935 
WorkingDirectory=/home/max/hashed/hashed-substrate
Restart=always
RestartSec=120

[Install]
WantedBy=multi-user.target
```

## SSL setup (firewall configuration)

```bash
# SSH was allowed beforehand the activation of the firewall as a security measure
sudo ufw allow 22/tcp
sudo ufw enable
# Proxy port
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
# For kusama
sudo ufw deny 30333/tcp
sudo ufw deny 9944/tcp
sudo ufw deny 9933/tcp
# For Liberland
sudo ufw deny 30334/tcp
sudo ufw deny 9945/tcp
sudo ufw deny 9934/tcp
# For hashed-chain
sudo ufw deny 30335/tcp
sudo ufw deny 9946/tcp
sudo ufw deny 9935/tcp

sudo ufw reload
sudo ufw verbose
```

```bash
sudo nano /etc/systemd/journald.conf
```

```bash
[Journal]
Storage=persistent
RateLimitIntervalSec=30s
RateLimitBurst=20000
SystemMaxUse=5G
SystemMaxFileSize=512M
SystemMaxFiles=100
```

```bash
sudo systemctl restart systemd-journald
```

## Nginx configuration

Every service must have a reverse-proxi configuration file:

```bash
cd /etc/nginx/sites-available
sudo touch /etc/nginx/sites-available/n1.liberland.network
sudo touch /etc/nginx/sites-available/liberland.network
sudo touch /etc/nginx/sites-available/backend.liberland.network
sudo touch /etc/nginx/sites-available/kusama.hashed.systems
sudo touch /etc/nginx/sites-available/n1.hashed.systems
```

It is crucial to replace the domain name and the assigned port (In case of being a susbtrate node config file, the port shall be the WS one, see [List of ports per project](#list-of-ports-per-project))

```bash
server {
        listen 80;
        listen [::]:80;

        server_name <domain-name>;

        location / {
                proxy_pass http://127.0.0.1:<Port || WS port>;
        }
}

```

```bash
#Create symbolic links for all the sites
sudo ln -s /etc/nginx/sites-available/n1.liberland.network /etc/nginx/sites-enabled/n1.liberland.network
sudo ln -s /etc/nginx/sites-available/liberland.network /etc/nginx/sites-enabled/liberland.network
sudo ln -s /etc/nginx/sites-available/backend.liberland.network /etc/nginx/sites-enabled/backend.liberland.network
sudo ln -s /etc/nginx/sites-available/kusama.hashed.systems /etc/nginx/sites-enabled/kusama.hashed.systems
sudo ln -s /etc/nginx/sites-available/n1.hashed.systems /etc/nginx/sites-enabled/n1.hashed.systems
```

## Certbot Certificates

```bash
sudo certbot -d '*.liberland.network' --nginx
sudo certbot -d 'liberland.network' --nginx
sudo certbot -d 'kusama.hashed.systems' --nginx
```

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
| p2p port | `--port` | 30334 |
| WebSocket port | `--ws-port` | 9945 |
| rcp port | `--rpc-port` | 9934 |

#### Hashed (increment by 2)

| Service | flag/option | port |
|---------|-------------|------|
| p2p port | `--port` | 30335 |
| WebSocket port | `--ws-port` | 9946 |
| rcp port | `--rpc-port` | 9935 |

#### Liberland frontend: 8080
#### Liberland backend: 3000

### Useful commands

```bash
# See open ports
sudo lsof -i -P -n | grep LISTEN
```
