FROM debian:jessie AS builder

# You'll need to change `libmysqlclient-dev` to `libpq-dev` if you're using Postgres
#RUN apt-get update && apt-get install -y curl libmysqlclient-dev build-essential
RUN apt-get update && apt-get install -y curl build-essential

RUN  apt-get install -y pkg-config libssl-dev

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly

ENV PATH="/root/.cargo/bin:${PATH}"

# The DATABASE_URL environment variable must be set at build time to a database which it can prepare queries against; the database does not have to contain any data but must be the same kind (MySQL, Postgres, etc.) and have the same schema as the database you will be connecting to at runtime.
#ENV DATABASE_URL="postgresql://postgres:test@192.168.50.71:5431/licensing"

ADD . ./

RUN cargo build --release

FROM debian:jessie


#RUN apt-get update && apt-get install -y libmysqlclient-dev
RUN apt-get update && apt-get install -y

RUN  apt-get install -y pkg-config libssl-dev

WORKDIR /root

COPY --from=builder \
  /target/release/pdemo \
  /root/

# COPY firebase key
COPY --from=builder sa.json ./sa.json
# COPY env
COPY --from=builder .env ./.env

WORKDIR /root
CMD /root/pdemo