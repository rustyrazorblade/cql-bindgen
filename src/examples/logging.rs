//#![feature(plugin)]
//#![plugin(clippy)]
#![allow(float_cmp)]
#[macro_use]
extern crate log;
extern crate cql_bindgen;
extern crate num;
extern crate libc;
extern crate env_logger;

mod examples_util;
use examples_util::*;

use std::ptr;
use std::env;
use std::ffi::CStr;

use libc::types::common::c95::c_void;

use cql_bindgen::*;

extern "C" fn on_log(message: *const CassLogMessage, data: *mut c_void) {
    unsafe {
        let _ = data;
        let message = &*message;
        info!(target: "cass_log", "{:?}.{:?} [{:?}] ({:?}:{:?}:{:?}) {:?}",
            message.time_ms / 1000,
            message.time_ms % 1000,
            message.severity,
            message.file,
            message.line,
            message.function,
            CStr::from_ptr(message.message[..].as_ptr())
        );
    }
}

fn main() {
    unsafe {
        env::set_var("RUST_LOG", "info");
        env_logger::init().unwrap();
        //Log configuration *MUST* be done before any other driver call
        cass_log_set_level(CASS_LOG_INFO);
        cass_log_set_callback(Some(on_log), ptr::null_mut());

        let cluster = create_cluster().unwrap();
        let session = &mut*cass_session_new();

        connect_session(session, cluster).unwrap();

        let close_future = cass_session_close(session);
        cass_future_wait(close_future);
        cass_future_free(close_future);

        cass_cluster_free(cluster);
        cass_session_free(session);

    }
}
