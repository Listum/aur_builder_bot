# Aur Builder Bot

## Описание

Реинкарнация мёртвого питон проекта на расте.

Телеграм бот для сёрфинга/парсинга AUR и компиляции пакетов, а потом добавления их в личный репозиторий в виде бинарников.


## Настройка

### Standalone

1. Укажите токен бота телеграм
```bash
# Unix-like
$ export TELOXIDE_TOKEN=<Your token here>

# Windows command line
$ set TELOXIDE_TOKEN=<Your token here>

# Windows PowerShell
$ $env:TELOXIDE_TOKEN=<Your token here>
```

3. Запустите бота
```bash
./aur_builder_bot
```
4. Создайте симлинк до вашего локального репозитория
```bash
ln -s /path/to/bot_dir/repo /path/to/repo
```

### Docker

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

1. Поиск пакетов в AUR
`/search <название пакета> <кол-во отображаемых пакетов 1-255>`

2. Добавление пакетов в репозиторий
`/upload <название пакета>`


## Сторонние библиотеки

- [Teloxide](https://crates.io/crates/teloxide)
- [Git2](https://crates.io/crates/git2)
- [Aur_rpc](https://crates.io/crates/aur-rpc)
- [Glob](https://crates.io/crates/glob)
