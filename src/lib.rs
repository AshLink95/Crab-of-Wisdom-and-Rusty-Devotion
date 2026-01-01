#![allow(dead_code)]
mod trie;
mod db;
mod manager;
use crate::manager::TrieManager;

use mlua::prelude::*;

// All that is needed in lua is the complete functionality
#[mlua::lua_module]
fn libcoward(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    
    let manager = TrieManager::new(db::DB_PATH).unwrap();
    let complete = lua.create_function(move |this, a: String| Ok({
        let completions = manager.complete(&a);
        let lua_ret = this.create_sequence_from(completions)?;
        Ok::<mlua::Table, LuaError>(lua_ret)
    }))?;
    exports.set("complete", complete)?;
    
    Ok(exports)
}
