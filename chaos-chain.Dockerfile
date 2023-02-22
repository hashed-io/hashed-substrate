FROM paritytech/ci-linux:production as build

WORKDIR /var/www

RUN git clone https://github.com/hashed-io/hashed-substrate.git && cd hashed-substrate && git checkout feature/chaos-chain-new-deployment && cargo build --release
WORKDIR /var/www/hashed-substrate

FROM paritytech/ci-linux:production

WORKDIR /var/www/hashed-substrate
COPY --from=build /var/www/hashed-substrate/target ./target
COPY --from=build /var/www/hashed-substrate/resources ./resources
COPY --from=build /var/www/hashed-substrate/scripts ./scripts

