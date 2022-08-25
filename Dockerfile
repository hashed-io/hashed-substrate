FROM paritytech/ci-linux:production

# COPY . /var/www/hashed
WORKDIR /var/www/

RUN git clone https://github.com/hashed-io/hashed-substrate.git

WORKDIR /var/www/hashed-substrate/

RUN git checkout feature/collator

COPY ./scripts/start_node.sh /var/www/hashed-substrate/scripts/start_node.sh
COPY ./scripts/start_collator.sh /var/www/hashed-substrate/scripts/start_collator.sh

EXPOSE 30333 40333 9933 9944 9946

CMD [ "/var/www/hashed-substrate/scripts/start_collator.sh" ]