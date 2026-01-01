# coward.nvim
coward.nvim is a neovim plugin for natural language autocompletion built on Rust.

This project also offers a CLI tool for manual addition and inspection.

## Dependencies
* [nvim-cmp](https://github.com/hrsh7th/nvim-cmp)
* [Rust](https://rust-lang.org)
* [SQLite](https://www.sqlite.org/)

## Setup
On any platform, you can run the provided `install.py` python script to setup this plugin and install its binary:
```bash
python install.py
```

### Manual setup
To manually setup this package, you need to first build the rust library and binary from source using
```bash
RUSTFLAGS="-C target-cpu=native -C force-frame-pointers=no" cargo build --release
```

Then, you need to generate the database. The default database path is `coward.db` in the repo root (same directory as `README.md` and `Cargo.toml`). The path variable that stores it is `db::DB_PATH` and it's in `src/db.rs`.

Using the [english words repo](https://github.com/dwyl/english-words/blob/master/words_alpha.txt), I created the file `insert_english.txt` and provided it along with the source code. All you need to do to use it is run these 2 commands:
```bash
sed "s/.*/INSERT OR IGNORE INTO words (word) VALUES ('&');/" insert_english.txt > insert_english.sql
sqlite3 coward.db < insert_english.sql
```
