FROM paritytech/ci-linux:production

WORKDIR /var/www

RUN git clone https://github.com/hashed-io/hashed-substrate.git && cd hashed-substrate && git checkout docker-mainnet && cargo build --release

WORKDIR /var/www/hashed-substrate
