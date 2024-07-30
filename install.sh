#!/usr/bin/env bash

npm install --global \
  lefthook@1.6.9 \
  remark-cli@12.0.1 \
  remark-validate-links@8.0.0 \
  remark-lint-no-dead-urls@1.1.0 \
  @commitlint/cli@19.3.0 \
  @commitlint/config-conventional@19.2.2

cargo install --git https://github.com/DenisGorbachev/cargo-doc2readme --branch dev
cargo install cargo-machete

# Install yj
curl -L https://github.com/sclevine/yj/releases/download/v5.1.0/yj-linux-amd64 -o /tmp/yj
chmod +x /tmp/yj
sudo mv /tmp/yj /usr/local/bin/yj
