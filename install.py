#!/usr/bin/env python

import subprocess
import sqlite3
import shutil
import os

# Optimized build with Rust flags
subprocess.run(
    ["cargo", "build", "--release"],
    env={**dict(os.environ), "RUSTFLAGS": "-C target-cpu=native -C force-frame-pointers=no"},
    check=True
)

# Copy the binary into plugin root
shutil.copy( "target/release/coward", "./cowardCLI")

# Read input file and insert into SQLite
db = sqlite3.connect("coward.db")
db.execute("""CREATE TABLE IF NOT EXISTS words (
                word TEXT PRIMARY KEY,
                added_at INTEGER DEFAULT (strftime('%s', 'now'))
)""")
with open("insert_english.txt") as f:
    db.executemany(
        "INSERT OR IGNORE INTO words (word) VALUES (?)",
        ((line.strip(),) for line in f if line.strip())
    )
db.commit()
db.close()
