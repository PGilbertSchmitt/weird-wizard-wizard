# Weird Wizard Wizard

A "Wizard" for creating characters for the Shadow of the Weird Wizard TTRPG

## How to develop

To run the app, run:

```bash
$ pnpm tauri dev
```

SQLX can check queries at compile time, but it requires having an up-to-date database that it can access. Since I'm using Sqlite, it's trivial to set up. It's been configured to look for `./src-tauri/test.db`. Since there's only 1 migration, all that needs to be done is to run this from the `./src-tauri/` directory:

```bash
$ sqlite3 test.db < db/migrations/20260627_initial_schema.sql
```

If you ever update the schema by creating new migrations, run them against the `test.db` as well.
