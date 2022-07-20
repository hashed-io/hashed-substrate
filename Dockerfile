FROM paritytech/ci-linux:production

WORKDIR /var/www/

RUN git clone https://github.com/hashed-io/hashed-substrate.git

WORKDIR /var/www/hashed-substrate/

RUN git checkout docker-mainnet

EXPOSE 30333 9933 9944

CMD [ "bash", "echo $MNEMO"]