# blogging_backend
Just mimicing the blogging site

## Need to have
1. sqlite3 and libsqlite3-dev: use ``sudo apt install sqlite3 && sudo apt install libsqlite3-dev`` to install in ubuntu
2. cargo: a rust build tool
3. diesel_cli: use `cargo install diesel_cli --no-default-features --features sqlite to install diesel_cli` for sqlite

## To run

1. diesel migration run: to create the database and the tables
2. cargo run: to run the webserver
