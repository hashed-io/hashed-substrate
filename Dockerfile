FROM paritytech/ci-linux:production

COPY . /var/www/hashed

WORKDIR /var/www/hashed

EXPOSE 30333 9933 9944

CMD [ "bash", "echo $MNEMO"]