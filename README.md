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
ln -s /path/to/bot/repo /path/to/repo
```

### Docker

#### В разработке

## Использоание

1. Поиск пакетов в AUR
`/search <название пакета>`

2. Добавление пакетов в репозиторий
`/upload <название пакета>`


## Сторонние библиотеки

<ul>
  <li>[Teloxide](https://crates.io/crates/teloxide)</li>
  <li>[Git2](https://crates.io/crates/git2)</li>
  <li>[Aur_rpc](https://crates.io/crates/aur-rpc)</li>
  <li>[Glob](https://crates.io/crates/glob)</li>
</ul> 