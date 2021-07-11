use rlua::{Function, Lua, Result as LuaResult};
use std::io::BufReader;
use std::path::PathBuf;
use std::{fs::File, io::Read};

fn load_from_file(path: PathBuf) -> Result<String, std::io::Error> {
    let f = File::open(&path)?;
    let mut f = BufReader::new(f);
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> LuaResult<()> {
    let lua = Lua::new();

    let p = PathBuf::from("./src/bin/script.lua");
    let script = load_from_file(p).expect("cant load script file");

    lua.context(|lua_ctx| {
        lua_ctx.load(&script).set_name("example")?.exec()?;

        let item_func: Function = lua_ctx.globals().get("process_item")?;
        item_func.call::<_, ()>(())?;
        Ok(())
    })?;

    Ok(())
}
