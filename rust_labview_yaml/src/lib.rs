extern crate libc;
extern crate yaml_rust;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::fs;
use yaml_rust::{YamlLoader};

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn c_load_unload_no_panic(){
        let c_string_path = CString::new(String::from("C:/sandbox/rust-labview/rust_labview_yaml/test_configuration.yml")).unwrap();
        let c_string_ptr = c_string_path.as_ptr(); 

        let daq_configs_ptr = load_daq_configurations(c_string_ptr);

        unload_daq_configurations(daq_configs_ptr);
    }

    #[test]
    fn c_three_configurations_load(){
        let daq_configs_ptr = load_config_test_setup();

        let count = get_configuration_count(daq_configs_ptr);
        assert_eq!(count, 3);
        unload_daq_configurations(daq_configs_ptr);
    }

    #[test]
    fn c_get_min_voltage(){
        let daq_configs_ptr = load_config_test_setup();
        
        let min = get_min_voltage(daq_configs_ptr, 0);
        assert_eq!(min, -5);
        unload_daq_configurations(daq_configs_ptr);
    }

    #[test]
    fn c_get_max_voltage(){
        let daq_configs_ptr = load_config_test_setup();
        
        let max = get_max_voltage(daq_configs_ptr, 2);
        assert_eq!(max, 1);
        unload_daq_configurations(daq_configs_ptr);
    }

    #[test]
    fn c_get_sample_rate(){
        let daq_configs_ptr = load_config_test_setup();
        
        let rate = get_sample_rate_hz(daq_configs_ptr, 1);
        assert_eq!(rate, 1000);
        unload_daq_configurations(daq_configs_ptr);
    }

     #[test]
    fn c_get_hardware(){
        let daq_configs_ptr = load_config_test_setup();
        
        let returned_chars = get_hardware_channels(daq_configs_ptr, 2);
        unsafe{
            let c_hardware_string = CString::from_raw(returned_chars);
            let hardware_channels = c_hardware_string.to_str().unwrap();
            assert_eq!(hardware_channels, "cDAQ1Mod3/ai0:15");
        }
        unload_daq_configurations(daq_configs_ptr);
    }

    fn load_config_test_setup() -> *mut DaqConfigurations{
        let c_string_path = CString::new(String::from("C:/sandbox/rust-labview/rust_labview_yaml/test_configuration.yml")).unwrap();
        let c_string_ptr = c_string_path.as_ptr(); 
        load_daq_configurations(c_string_ptr)
    }
}

// Uses the on the Rust Omnibus object approach:
// http://jakegoulding.com/rust-ffi-omnibus/objects/

pub struct DaqConfigurations {
    configs: Vec<yaml_rust::Yaml>,
}

impl DaqConfigurations {
    fn new() -> DaqConfigurations {
        DaqConfigurations {
            configs: Vec::<yaml_rust::Yaml>::new(),
        }
    }

    fn load_configuration_file(&mut self, filepath: String){
        let file_contents = fs::read_to_string(filepath).unwrap();
        let docs = YamlLoader::load_from_str(&file_contents).unwrap();
        let doc = &docs[0];
        self.configs = doc["cDAQ_configurations"].as_vec().unwrap().to_vec();
    }

    fn get_count(& self) -> u32 {
        let length = self.configs.len();
        length as u32
    }

    fn get_min(& self, index: u32) -> i64{
        let index = index as usize;
        let config = &self.configs[index];
        config["min_voltage"].as_i64().unwrap()
    }

    fn get_max(& self, index: u32) -> i64{
        let index = index as usize;
        let config = &self.configs[index];
        config["max_voltage"].as_i64().unwrap()
    }

    fn get_rate(& self, index: u32) -> i64{
        let index = index as usize;
        let config = &self.configs[index];
        config["sample_rate_Hz"].as_i64().unwrap()
    }

    fn get_hw(& self, index: u32) -> String{
        let index = index as usize;
        let config = &self.configs[index];
        config["hardware_channels"].as_str().unwrap().to_string()
    }
}

#[no_mangle]
pub extern "C" fn load_daq_configurations(filepath: *const c_char) -> *mut DaqConfigurations{
    let filepath = unsafe {
        assert!(!filepath.is_null());
        CStr::from_ptr(filepath)
    };
    let filepath_string = filepath.to_str().unwrap().to_string();
    let mut configurations = DaqConfigurations::new();
    configurations.load_configuration_file(filepath_string);
    Box::into_raw(Box::new(configurations))
}

#[no_mangle]
pub extern "C" fn unload_daq_configurations(ptr: *mut DaqConfigurations) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn get_configuration_count(ptr: *mut DaqConfigurations) -> u32 {
    let configs = unsafe{
        assert!(!ptr.is_null());
        &mut *ptr
    };
    configs.get_count()
}

#[no_mangle]
pub extern "C" fn get_min_voltage(ptr: *mut DaqConfigurations, index: u32) -> i64 {
    let configs = unsafe{
        assert!(!ptr.is_null());
        &mut *ptr
    };
    configs.get_min(index)
}

#[no_mangle]
pub extern "C" fn get_max_voltage(ptr: *mut DaqConfigurations, index: u32) -> i64 {
    let configs = unsafe{
        assert!(!ptr.is_null());
        &mut *ptr
    };
    configs.get_max(index)
}

#[no_mangle]
pub extern "C" fn get_sample_rate_hz(ptr: *mut DaqConfigurations, index: u32) -> i64 {
    let configs = unsafe{
        assert!(!ptr.is_null());
        &mut *ptr
    };
    configs.get_rate(index)
}

#[no_mangle]
pub extern "C" fn get_hardware_channels(ptr: *mut DaqConfigurations, index: u32) -> *mut c_char {
    let configs = unsafe{
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let hardware_string = configs.get_hw(index);
    CString::new(hardware_string).unwrap().into_raw()
}
