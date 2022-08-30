FROM paritytech/ci-linux:production

WORKDIR /
RUN git clone https://github.com/hashed-io/hashed-substrate.git && cd hashed-substrate && git checkout feature/collator && cargo build --release

# COPY ./target/release/hashed-parachain ./.
# COPY ./resources/* resources/.
# COPY ./scripts/start_collator.sh scripts/.

EXPOSE 30333 40333 9933 9944 9946

CMD [ "scripts/start_collator.sh" ]