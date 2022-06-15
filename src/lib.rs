mod error;
mod limit;

pub use error::*;
pub use limit::*;

#[allow(clippy::all)]
#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));

    pub type sqlite3_destructor_type =
        ::std::option::Option<unsafe extern "C" fn(_: *mut ::std::os::raw::c_void)>;

    pub fn SQLITE_STATIC() -> sqlite3_destructor_type {
        None
    }

    pub fn SQLITE_TRANSIENT() -> sqlite3_destructor_type {
        Some(unsafe { ::std::mem::transmute(-1_isize) })
    }

    pub type sqlite3_index_constraint = sqlite3_index_info_sqlite3_index_constraint;
    pub type sqlite3_index_constraint_usage = sqlite3_index_info_sqlite3_index_constraint_usage;

    impl Default for sqlite3_vtab {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }

    impl Default for sqlite3_vtab_cursor {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
}

pub use bindings::*;
