FROM paritytech/ci-linux:production

# COPY . /var/www/hashed
WORKDIR /var/www/

RUN git clone https://github.com/hashed-io/hashed-substrate.git

WORKDIR /var/www/hashed-substrate/

RUN git checkout e4beec4fae5d0c7d23d0f1950b6f7f49e1bbf95a

COPY ./scripts/start_node.sh /var/www/hashed-substrate/scripts/start_node.sh

EXPOSE 30333 9933 9944

CMD [ "bash", "echo $MNEMO"]