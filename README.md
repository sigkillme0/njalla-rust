# njalla

rust toolkit for the [njal.la](https://njal.la) api. library + cli.

## install

```sh
cargo install --path .
```

to update after a `git pull`:

```sh
cargo install --path . --force
```

to uninstall:

```sh
cargo uninstall njalla
```

## config

generate an api token from your [njalla account settings](https://njal.la/settings/api/), then either:

```sh
# option a: shell environment (recommended)
echo 'export NJALLA_API_TOKEN=your-token' >> ~/.bashrc

# option b: config file
mkdir -p ~/.config/njalla
echo 'NJALLA_API_TOKEN=your-token' > ~/.config/njalla/.env
```

lookup order: env var → `.env` in cwd → `~/.config/njalla/.env`

## usage

```sh
# domains
njalla domain list
njalla domain get example.com
njalla domain find mysite
njalla domain register example.com 2
njalla domain check-task <task-id>

# dns records
njalla record list example.com
njalla record add example.com -n "@" -t A -c "1.2.3.4" --ttl 3600
njalla record add example.com -n "@" -t MX -c "mail.example.com" --ttl 3600 -p 10
njalla record edit example.com <id> --content "5.6.7.8"
njalla record remove example.com <id>

# servers
njalla server list
njalla server images
njalla server types
njalla server add mybox -t njalla1 -o ubuntu2404 -s "ssh-rsa ..." -m 1
njalla server stop <id>
njalla server start <id>
njalla server restart <id>
njalla server reset <id> -o debian13 -s "ssh-rsa ..." -t njalla2
njalla server remove <id>
```

`record edit` fetches the existing record and patches only the fields you pass — no need to re-specify everything.

all output is json.

## as a library

```rust
use njalla::{NjallaClient, NewRecord};

let client = NjallaClient::from_env()?;

let domains = client.list_domains().await?;
let records = client.list_records("example.com").await?;

let created = client.add_record("example.com", &NewRecord {
    name: "@".into(),
    record_type: "A".into(),
    content: "1.2.3.4".into(),
    ttl: 3600,
    priority: None,
}).await?;

client.remove_record("example.com", created.id.as_deref().unwrap()).await?;
```

## building from source

```sh
cargo build --release
```

requires rust 1.93+.

## license

[WTFPL](LICENSE)
