# Conexipro

## Requirements

- cargo install cargo-make
- cargo install cargo-watch
- [diesel](https://diesel.rs/guides/getting-started)

## Install diesel CLI

```bash
# Linux/MacOS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh

# Windows
powershell -c "irm https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.ps1 | iex"
```

## Running on development

```bash
# Copy environment variables 
cp .env.example .env

# Run with option 1
cargo make dev

# Run with option 2
cargo watch -c -w src -x run
```

## Build project

```bash
# build rust application
cargo build --release

# run the application
./target/release/conexipro
```

## Migration

***It does not required to run migration on production. However we might need to set `APP_ENV=production` in environment variables.***

```bash
# run
diesel migration run

# rollback
diesel migration revert
```

## Use pm2 to start the build application

```bash
# install pm2
npm install -g pm2

# run application
pm2 start

# check log
pm2 log conexipro

# check instances
pm2 list conexipro
```

## Generate JWT Secret

```bash
openssl rand -base64 48 | tr -dc 'a-zA-Z0-9' | fold -w 64 | head -n 1
```
