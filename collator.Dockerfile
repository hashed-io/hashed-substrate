FROM paritytech/ci-linux:production

WORKDIR /var/www
# TODO: have this dockerfile pull the hashed-substrate repo
COPY . ./hashed-substrate

WORKDIR /var/www/hashed-substrate
# this dir doesnt exists but zombienet script tries to create a file within
RUN mkdir /cfg

RUN cargo build --release

EXPOSE 30333 40333 9933 9944 9946

# add binary to docker image
RUN mv /var/www/hashed-substrate/target/release/hashed-parachain /usr/local/bin

# check if executable works in this container
RUN /usr/local/bin/hashed-parachain --version

ENTRYPOINT [ "hashed-parachain" ]