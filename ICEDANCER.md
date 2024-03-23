# Segfault fix

- Use stable commit 68a4471214cbca1ef9031410ce03f51960f44e2e
- Important-Compile the plugin with rust version given in CI, i.e. 1.69.0

# Setup

- Build with `cargo build --release`

- Postgres post-installation setup: we must set a password for default user `postgres` and switch to md5 auth

```sh
# set password for user postgres
sudo -u postgres psql
ALTER USER postgres PASSWORD 'password';

# edit auth file, change peer authentication to md5
# https://stackoverflow.com/a/18664239
sudo nano /etc/postgresql/16/main/pg_hba.conf

# restart postgres
sudo systemctl restart postgresql
```

- Postgres commands
  - Note: postgres-16 runs on port 5432 instead of 5433

```sh
# create DB solana
sudo -u postgres createdb solana

# run script to generate schema
psql -U postgres -W -d solana -f scripts/drop_schema.sql
psql -U postgres -W -d solana -f scripts/create_schema.sql


# run validator
solana-test-validator --geyser-plugin-config ./config.json
```


Test host

```sh
# works
psql -U postgres -p 5432 -h localhost -W -d solana

# works too
psql -U postgres -p 5432 -h /var/run/postgresql -W -d solana
```

## Reading data

- Logs are present in Transaction.meta.log_messages[]
