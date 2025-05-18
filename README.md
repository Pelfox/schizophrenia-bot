# Schizophrenia Bot

🤡 A simple Telegram bot for my friends' group.

### Running the bot

- Preferred way: use [Docker image](https://github.com/Pelfox/schizophrenia-bot/pkgs/container/schizophrenia-bot),
you can find Docker Compose configuration in the root of the repository.

- Building from source: clone the repository, then run `cargo check` to build
dependencies and `cargo run` to create a development build. To create a
production build, run `cargo build --release`.

> [!IMPORTANT]
> When building a Docker image, specify the platform `linux/amd64`, if you want
> to run this image on non-amd64 platform, i.e.:
> `docker build --platform linux/amd64 -t ghcr.io/pelfox/schizophrenia-bot:latest .`

See [.env.example](.env.example) for more information on environment variables.

### Running the migrations

If using Docker image, it contains migrations as a separate binary. Just run
`docker run --rm -e DATABASE_URL=postgres://postgres:postgres@postgres:5432/postgres
ghcr.io/pelfox/schizophrenia-bot:latest diesel migration run`.

When building manually, install [Diesel CLI](https://diesel.rs/guides/getting-started)
and run the following command: `diesel migration run`.

### LICENSE

This project is licensed with an [MIT license](./LICENSE). Read its rules and
regulations for more information on this subject.
