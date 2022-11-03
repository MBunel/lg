use crate::filter::Compare;
use crate::header::MyHeader;

use regex::Regex;
use semver::VersionReq;

#[derive(Debug)]
pub struct StringFilter {
    value: String,
    header_attribute: String,
}

impl StringFilter {
    pub fn new(input_string: &String, attribute_name: &String) -> StringFilter {
        StringFilter {
            value: input_string.to_string(),
            header_attribute: attribute_name.to_string(),
        }
    }
}

impl Compare for StringFilter {
    fn compare(&self, value: &MyHeader) -> bool {
        match self.header_attribute.as_str() {
            "gps_time_type" => match self.value.as_str() {
                "week" => !value.gps_time_type.is_standard(),
                "standard" => value.gps_time_type.is_standard(),
                _ => panic!("Gps time type error"),
            },
            "las_version" => {
                let req = VersionReq::parse(&self.value).unwrap();
                return req.matches(&value.version);
            }
            "guid" => {
                let re = Regex::new(&self.value).unwrap();
                re.is_match(&value.guid.to_string())
            }
            "generating_software" => {
                let re = Regex::new(&self.value).unwrap();
                re.is_match(&value.generating_software)
            }
            "system_identifier" => {
                let re = Regex::new(&self.value).unwrap();
                re.is_match(&value.system_identifier)
            }
            _ => false,
        }
    }
}
