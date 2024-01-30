# Aur Builder Bot

## Description

Reincarnation of a dead python project on rust.

Telegram bot for surfing/parsing AUR and compiling packages and then adding them to a personal repository as binary.

## Setup

### Standalone

1. Specify the token of the Telegram bot
```bash
$ export TELOXIDE_TOKEN=<Your token>
```
2. Specify a password for authorization
```bash
$ export PASS=<Password>
```
3. Add GPG key for signing (optional)
  - ```$ gpg --list-keys --keyid-format=long```
  - ```rsa4096/D8DDA4AE70FAD33E``` copy the key ID (here its: **D8DDA4AE70FAD33E**)
  - ```$ export GPGKEY=<ID ключа>```
4. Start the bot
```bash
./aur_builder_bot
```
5. Create a symlink to your local repository
```bash
ln -s /path/to/bot_dir/repo /path/to/repo
```

### Docker

GPG signing currently not working with docker setup

**docker-cli**
```bash
docker run \
 --name AUR_Builder_Bot \
 --restart=unless-stopped \
 -v /path/to/repo:/opt/aur_build/repo \
 -e TELOXIDE_TOKEN="<Your_token_here>"
 -d orudoca/aur_builder_bot:latest
```

**docker-compose**
```yml
services:
  aur_builder_bot:
    image: orudoca/aur_builder_bot:latest
    container_name: AUR_Builder_Bot
    volumes:
      - /path/to/repo/:/opt/aur_builder/repo
    restart: 'unless-stopped'
    environment:
      TELOXIDE_TOKEN: "<Your_token_here>"
```
```bash
docker compose up -d
```

## Использоание

1. Search for packages in AUR
`/search <package name> <number of packets displayed 1-255>`

2. Adding packages to the repository
`/upload <package name>`


## Third-party libraries

- [Teloxide](https://crates.io/crates/teloxide)
- [Git2](https://crates.io/crates/git2)
- [Aur_rpc](https://crates.io/crates/aur-rpc)
- [Glob](https://crates.io/crates/glob)
