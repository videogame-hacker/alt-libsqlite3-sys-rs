use std::{env, path::Path};

use bindgen::callbacks::{IntKind, ParseCallbacks};

#[derive(Debug)]
struct SqliteTypeChooser;

impl ParseCallbacks for SqliteTypeChooser {
    fn int_macro(&self, _name: &str, value: i64) -> Option<IntKind> {
        if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
            Some(IntKind::I32)
        } else {
            None
        }
    }
}

fn compile_args() -> Vec<String> {
    let mut args: Vec<String> = "-DSQLITE_CORE
	-DSQLITE_ENABLE_COLUMN_METADATA
	-DSQLITE_ENABLE_DBSTAT_VTAB
	-DSQLITE_ENABLE_FTS3
	-DSQLITE_ENABLE_FTS3_PARENTHESIS
    -DSQLITE_ENABLE_FTS4
	-DSQLITE_ENABLE_FTS5
	-DSQLITE_ENABLE_GEOPOLY
	-DSQLITE_ENABLE_RTREE
	-DSQLITE_ENABLE_UNLOCK_NOTIFY
	-DSQLITE_MAX_VARIABLE_NUMBER=250000
	-DSQLITE_SECURE_DELETE
	-DSQLITE_USE_URI
    -DSQLITE_DEFAULT_FOREIGN_KEYS=1
    -DSQLITE_DEFAULT_WAL_SYNCHRONOUS=1
    -DSQLITE_ENABLE_FTS4
    -DSQLITE_ENABLE_LOAD_EXTENSION=1
    -DSQLITE_ENABLE_MEMORY_MANAGEMENT
    -DSQLITE_ENABLE_STAT4
    -DSQLITE_SOUNDEX
    "
    .split_whitespace()
    .map(|s| s.to_string())
    .collect();

    if let Ok(extras) = env::var("LIBSQLITE3_FLAGS") {
        args.extend(extras.split_whitespace().map(|s| s.to_string()));
    }

    args
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("bindgen.rs");

    let mut bindings = bindgen::builder()
        .trust_clang_mangling(false)
        .header("sqlite3/sqlite3.h".to_string())
        .parse_callbacks(Box::new(SqliteTypeChooser))
        .rustfmt_bindings(true)
        .clang_args(compile_args());

    bindings = bindings
        .allowlist_var("SQLITE_.*")
        .allowlist_var("SQLITE3_.*")
        .allowlist_function("sqlite3_.*");

    bindings
        .generate()
        .expect("Could not run bindgen on sqlite3.h")
        .write_to_file(out_path)
        .expect("Could not write bindgen output");

    let mut cfg = cc::Build::new();
    cfg.file("sqlite3/sqlite3.c");
    for arg in compile_args() {
        cfg.flag(&arg);
    }
    cfg.compile("sqlite3");
}
