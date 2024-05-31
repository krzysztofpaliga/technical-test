FROM rust:bullseye

COPY . /technical-test/

WORKDIR /technical-test

RUN chmod u+x entrypoint.sh

ENTRYPOINT ["/technical-test/entrypoint.sh"]