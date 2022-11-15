FROM rust:1.65 as builder

COPY ./.nvmrc ./.nvmrc

# install node and npm for tailwind
ENV NODE_VERSION=18.12.1
RUN apt install -y curl
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
ENV NVM_DIR=/root/.nvm
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"
RUN node --version
RUN npm --version

COPY . .

RUN bin/task init && bin/task build:release

FROM debian:buster-slim

RUN apt update && apt install -y libssl1.1 && rm -rf /var/lib/apt/lists/*

COPY --from=builder ./target/release/server ./server

COPY --from=builder ./target/release/client ./client

CMD ["./server"]

