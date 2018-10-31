extern crate vcx;
extern crate indy;
extern crate libc;
#[macro_use]
extern crate log;

use self::libc::{c_void, c_char};
use std::ptr::null;
use vcx::api::logger::*;
#[allow(unused_imports)] use vcx::utils::logger::{LOGGER_STATE, LoggerState, LibvcxDefaultLogger};
use indy::wallet;
use vcx::utils::{cstring::CStringUtils, error::SUCCESS};
use vcx::api::logger::vcx_set_logger;
/// These tests can only be run individually as initing the log crate can happen
/// only once.
///
/// These tests usually need to be run manually to verify that the standard
/// logging is outputting to stdout.
#[allow(unused_imports)]
mod log_tests {
    use super::*;

    pub type EnabledCB = extern fn(context: *const c_void,
                                   level: u32,
                                   target: *const c_char) -> bool;

    pub type LogCB = extern fn(context: *const c_void,
                               level: u32,
                               target: *const c_char,
                               message: *const c_char,
                               module_path: *const c_char,
                               file: *const c_char,
                               line: u32);

    pub type FlushCB = extern fn(context: *const c_void);

    static mut COUNT: u32 = 0;
    extern fn custom_log(context: *const c_void,
                         level: u32,
                         target: *const c_char,
                         message: *const c_char,
                         module_path: *const c_char,
                         file: *const c_char,
                         line: u32) {
        let message = CStringUtils::c_str_to_string(message).unwrap();
        unsafe { COUNT = COUNT + 1 }
    }
    #[test]
    fn test_logging_default_debug() {
        // this test should output a single debug line
        // and a single info line (from the vcx_error_c_message call)
        use vcx::api::vcx::vcx_error_c_message;
        let pattern = CStringUtils::string_to_cstring("debug".to_string());
        assert_eq!(vcx_set_default_logger(pattern.as_ptr()), 0);
        debug!("testing debug");
        vcx_error_c_message(1000);

    }

    #[test]
    fn test_logging_default_is_warn() {
        use std::ptr::null_mut;
        // this test should output a single warning line
        assert_eq!(vcx_set_default_logger(null()), 0);
        unsafe { assert_eq!(LOGGER_STATE, LoggerState::Default); }
        warn!("testing warning");
    }

    #[test]
    fn test_logging_env_var() {
        // this test should output a single info line
        use std::env::set_var;
        set_var("RUST_LOG", "info");
        assert_eq!(vcx_set_default_logger(null()), 0);
        info!("testing info");
    }

    /// This test depends on some modifications to the indy code.
    /// By adding a indy_set_default_logger(null()) to the indy_create_wallet function,
    /// it tests that both calls to log::init an occur and not conflict
    #[ignore]
    #[test]
    fn test_works_with_libindy() {
        pub const DEFAULT_WALLET_CONFIG: &'static str = r#"{"id":"wallet_1","storage_type":"default"}"#;
        pub const WALLET_CREDENTIALS: &'static str = r#"{"key":"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY", "key_derivation_method":"RAW"}"#;
        wallet::Wallet::create(DEFAULT_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
        let pattern = CStringUtils::string_to_cstring("debug".to_string());
        assert_eq!(vcx_set_default_logger(pattern.as_ptr()), 0);
        debug!("testing debug");
        trace!("testing trace");
    }

    #[test]
    fn test_set_logger() {
        let err = vcx_set_logger(null(), None, Some(custom_log), None);
        debug!("testing debug");
        unsafe { assert_eq!(COUNT, 1); }

    }
}



