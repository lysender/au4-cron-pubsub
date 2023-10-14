# Cron Pub/Sub

A demo project that uses the following:
- clap for command line argument parsing
- TOML reader
- cron/scheduler which uses tokio
- Google Pub/Sub publisher client
- JWT for attaching message tokens (validated by the application only)

Scheduler that sends messages to Google Pub/Sub.

## Configuration

Copy the file `config-example.toml` as template for the configuration.

Required keys are the following:
- `tasks` - a list of tasks and their schedule
- `jwt_secret` - secret key used for application message tokens
- `pubsub` - configuration for Google Pub/Sub

## Build

For Windows, make sure you have a C++ compiler from Visual Studio.

Also setup and configure `vgpkg` to have `openssl`.

See this guide for more details:

https://gist.github.com/lysender/159820c2a9acd07a449ca2da598e5a81

For Mac and Linux, it should build as long as you have the usual build tools.

```
cargon build --release
```

## Usage

```
cron-pubsub -c /path/to/config.toml
```

