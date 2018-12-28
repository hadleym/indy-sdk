extern crate vcx;
#[macro_use]
extern crate serde_json;
extern crate libc;
extern crate rand;
use self::libc::c_char;
use vcx::api::{
    vcx::vcx_init_with_config,
    wallet::{vcx_wallet_add_record,
             vcx_wallet_add_record_tags,
             vcx_wallet_get_record
    },
    utils::vcx_provision_agent,
    return_types_u32,
};

use vcx::utils::cstring::CStringUtils;
use std::ffi::CString;
use std::time::Duration;
use vcx::api::logger::vcx_set_default_logger;
extern {
    fn indy_set_default_logger(pattern: *const c_char) -> i32;
}

mod non_secrets_test {
    use super::*;
//    pub const AGENCY_ENDPOINT: &'static str = "http://localhost:8080";
//    pub const AGENCY_DID: &'static str = "VsKV7grR1BUE29mG2Fm2kX";
//    pub const AGENCY_VERKEY: &'static str = "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR";

    fn create_test_vcx_config(config_string: &str) -> String {
        let mut config_json: serde_json::Value = serde_json::from_str(config_string).unwrap();
        config_json["genesis_path"] = json!("/home/mark/genesisfiles/qarc.txn");
        config_json["institution_logo_url"] = json!("https://robohash.org/ycalwb");
        config_json["institution_name"] = json!("Evernym Enterprise");
        assert_eq!(config_json["genesis_path"], json!("/home/mark/genesisfiles/qarc.txn"));
        assert_eq!(config_json["agency_did"], "FtdtGa9Hua1ZYF874yK6iC");
        config_json.to_string()
    }

    #[test]
    fn test_provision_agent() {
        assert_eq!(0, vcx_set_default_logger(CStringUtils::string_to_cstring("debug".to_string()).as_ptr()));
        let json_string = r#"{"agency_url":"http://easq002.pqa.evernym.com","agency_did":"FtdtGa9Hua1ZYF874yK6iC","agency_verkey":"97iEbD8rS3p51DzmLc4orS9AsE1YFnRZCVCEAUqfsj8D","wallet_name":"test_provision_agent","agent_seed":null,"enterprise_seed":null,"wallet_key":"key"}"#;
        let c_json = CString::new(json_string).unwrap().into_raw();
        let result = vcx_provision_agent(c_json);
        let c_res = CStringUtils::c_str_to_string(result).unwrap();
        println!("config: {:?}", c_res);
        let _vcx_config = json!(c_res);
        // init vcx
        // add record
    }

    pub fn enable_libindy_logging(level: &str) -> Result<(), u32> {
        let _level = CStringUtils::string_to_cstring(level.to_string());
        let res = unsafe {indy_set_default_logger(_level.as_ptr())};
        match res {
            0 => Ok(()),
            e => Err(e as u32)
        }

    }
    #[test]
    fn test_enable_libindy_logging() {
        enable_libindy_logging("trace").unwrap();
    }
    fn logging() {
        enable_libindy_logging("trace").unwrap();
        assert_eq!(0, vcx_set_default_logger(CStringUtils::string_to_cstring("trace".to_string()).as_ptr()));
    }
    #[test]
    fn test_create_vcx_config() {
        use rand;
        logging();
        let config = "{\"agency_did\":\"FtdtGa9Hua1ZYF874yK6iC\",\"agency_endpoint\":\"http://easq002.pqa.evernym.com\",\"agency_verkey\":\"97iEbD8rS3p51DzmLc4orS9AsE1YFnRZCVCEAUqfsj8D\",\"genesis_path\":\"<CHANGE_ME>\",\"institution_did\":\"2SwNfx8kDBfkLdXCs1Kxkx\",\"institution_logo_url\":\"<CHANGE_ME>\",\"institution_name\":\"<CHANGE_ME>\",\"institution_verkey\":\"nkRtXjnuMHd5EcTTVZ1y2FZYUsjWqNDqpG54ac1N9ig\",\"remote_to_sdk_did\":\"REUNfzcgtSvf5RLJS3RRG1\",\"remote_to_sdk_verkey\":\"ED2jtFdfeZAmaA5zDegTUE4X32wTJEqiewDLX8GYegrq\",\"sdk_to_remote_did\":\"DhWs6q7RiwqpDY4RLWQSQf\",\"sdk_to_remote_verkey\":\"7vRgtWudKPPzraWj6T4B3PpRr5qZwkMpJLm55PvuUvTJ\",\"wallet_key\":\"key\",\"wallet_name\":\"test_provision_agent\"}";
        let config_string = create_test_vcx_config(config);

        // init
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_init_with_config(cb.command_handle, CStringUtils::string_to_cstring(config_string).as_ptr(), Some(cb.get_callback())), 0);
        cb.receive(Some(Duration::from_secs(20))).unwrap();

        // add a record

        let cb = return_types_u32::Return_U32::new().unwrap();
        let r = rand::random::<u32>().to_string();
        let type_ = CStringUtils::string_to_cstring("agencydata".to_string());
        let id = CStringUtils::string_to_cstring(format!("agency_endpoint{}", r).to_string());
        let value = CStringUtils::string_to_cstring("http://easq002.pqa.evernym.com".to_string());
        let tags = CStringUtils::string_to_cstring(r#"{"tagName1":"consumer"}"#.to_string());
        assert_eq!(vcx_wallet_add_record(cb.command_handle, type_.as_ptr(), id.as_ptr(), value.as_ptr(), tags.as_ptr(), Some(cb.get_callback())), 0);

        cb.receive(Some(Duration::from_secs(10))).unwrap();
//        let cb = return_types_u32::Return_U32::new().unwrap();
//        assert_eq!(vcx_wallet_add_record_tags(cb.command_handle, type_.as_ptr(), id.as_ptr(), tags.as_ptr(), Some(cb.get_callback())), 0);
//        cb.receive(Some(Duration::from_secs(10))).unwrap();
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let options = CStringUtils::string_to_cstring("{}".to_string());
        assert_eq!(vcx_wallet_get_record(cb.command_handle, type_.as_ptr(), id.as_ptr(),options.as_ptr(), Some(cb.get_callback())), 0);
        let record = cb.receive(Some(Duration::from_secs(10))).unwrap();
        assert_eq!(record.unwrap(), "A FOOBAR RECORD");

    }

    #[test]
    fn test_generate_random_number() {
        use rand;
        let r = rand::random::<u32>();
        let anotherRandom = rand::random::<u32>();
        assert_ne!(r, anotherRandom);
    }

}
