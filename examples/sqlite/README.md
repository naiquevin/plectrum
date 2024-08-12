# Sqlite example

Example to demonstrate using `plectrum` with an sqlite database.

To run this example, first you'll need to create the sqlite db from
the provided schema. The `database/create-db` script has been provided
for this.

```bash
cd database
./create-db
```

The above command will create an sqlite database located at
[database/todos.db](database/todos.db). Note that this binary file is
gitignored.

You may run the binary crate from the current dir as follows,

```bash
cargo run
```

Or from the root of this git repo as follows,

```bash
PLECTRUM_SQLITE_DB=examples/sqlite/database/todos.db cargo run -p sqlite
```


