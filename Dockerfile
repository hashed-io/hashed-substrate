FROM paritytech/ci-linux:production

WORKDIR /var/www/

RUN git clone https://github.com/hashed-io/hashed-substrate.git

WORKDIR /var/www/hashed-substrate/

RUN git checkout feature/collator
RUN cargo build --release

# COPY ./target/release/hashed-parachain /var/www/hashed-substrate/target/release/hashed-parachain
# COPY ./resources/* /var/www/hashed-substrate/resources/.
# COPY ./scripts/start_node.sh /var/www/hashed-substrate/scripts/start_node.sh
# COPY ./scripts/start_collator.sh /var/www/hashed-substrate/scripts/start_collator.sh

EXPOSE 30333 40333 9933 9944 9946

CMD [ "/var/www/hashed-substrate/scripts/start_collator.sh" ]