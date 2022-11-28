FROM paritytech/ci-linux:production

# this dir doesnt exists but zombienet script tries to create a file within
RUN mkdir /cfg

WORKDIR /var/www

RUN git clone https://github.com/hashed-io/hashed-substrate.git

WORKDIR /var/www/hashed-substrate
# change to main or develop
RUN git checkout feature/hashed-chain-spec && cargo build --release

EXPOSE 30333 40333 9933 9944 9946

# add binary to docker image
RUN mv /var/www/hashed-substrate/target/release/hashed-parachain /usr/local/bin

# check if executable works in this container
RUN /usr/local/bin/hashed-parachain --version
# ENTRYPOINT allows to add parameters/flags via k8 manifests
ENTRYPOINT [ "hashed-parachain" ]