[![Neovim](https://img.shields.io/badge/neovim-0.11.5-green.svg)](https://neovim.io/)
[![LuaJIT](https://img.shields.io/badge/luajit-2.1-blue.svg)](https://luajit.org/)
[![Rust](https://img.shields.io/badge/rust-1.92-orange.svg)](https://www.rust-lang.org/)
# <div align=center>coward.nvim</div>
coward.nvim is a neovim plugin for natural language autocompletion built on Rust. It stands for "Crab of Wisdom and Rusted Devotion".

This plugin also offers a CLI tool for manual addition and inspection as well as 370k+ english words from the get-go.

## Dependencies
* [Rust](https://rust-lang.org)
* [SQLite](https://www.sqlite.org/)
* [nvim-cmp](https://github.com/hrsh7th/nvim-cmp)

## Implementation
This plugin uses a [trie](https://en.wikipedia.org/wiki/Trie) to easily store and access words. This trie is implemented in Rust and, upon loading, accesses a SQLite database where all words are stored. This implementation makes this plugin as fast and bloat-free as possible.

## Installation
On any platform, you can run the provided `install.py` python script to setup this plugin and install its binary. The whole procedure would look something like
```bash
git clone https://github.com/AshLink95/coward.nvim.git
cd coward.nvim
./install.py
```

You will find the file `insert_english.txt` which I created using the [english words repo](https://github.com/dwyl/english-words/blob/master/words_alpha.txt). That contains all english words. However, You can define the words you want your autocomplete to recognize by adding onto that file. You can even create your own but you have to modify the installation script.

## Setup and Configuration
In your nvim-cmp setup, make sure you add `{name = 'coward'}` to `require('cmp').config.sources({...})`.

You can setup the maximum items displayed (to 10, for example) by adding the following to your `init.lua`:
```lua
require('coward').setup({ max_items = 10 })
```
> Note that this the default `max_items` is 10

## CLI usage
You can manually inspect your DB by using
```bash
./cowardCLI
```
Once there, you'll know your way around. It's an intuitive CLI tool.

## Direct SQLite database manipulation
Keep in mind that this is a SQLite database. You can directly modify elements of the database and the programs will still function as intended. By default, the database is `coward.db` in the plugin root.
