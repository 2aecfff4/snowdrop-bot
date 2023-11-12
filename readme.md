# Snowdrop bot
An old Telegram bot that counted the number of messages sent in a group chat and sent daily statistics.

The bot was intended to be run on Heroku with a PostgreSQL database.


# How to build
`cargo build --release`


# How to run
In order for it to work properly, the following four environment variables must be set:
- `SERVER_ADDRESS`
- `BIND_PORT`
- `DATABASE_URL` 
- `BOT_TOKEN`

Example parameters can be found in the `.env` file.


# How it works
This bot receives messages through Telegram's webhook, which can be set up using `@BotFather`.


