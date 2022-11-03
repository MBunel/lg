use crate::filter::Compare;
use crate::header::MyHeader;

#[derive(Debug)]
pub struct ListFilter {
    value: Vec<u8>,
    header_attribute: String,
}

impl ListFilter {
    pub fn new(value_list: Vec<u8>, attribute_name: &String) -> ListFilter {
        ListFilter {
            value: value_list,
            header_attribute: attribute_name.to_string(),
        }
    }
}

impl Compare for ListFilter {
    fn compare(&self, value: &MyHeader) -> bool {
        let pf = value.point_format.to_u8().unwrap();
        self.value.contains(&pf)
    }
}
