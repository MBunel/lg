use crate::filter::Compare;
use crate::header::MyHeader;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct NumericFilter {
    operator: String,
    header_attribute: String,
    value: f64,
}

impl NumericFilter {
    pub fn new(input_string: &String, attribute_name: &String) -> NumericFilter {
        let re: Regex =
            Regex::new(r"(?P<operator>=|<=|>=|<|>|!=)(?P<value>[0-9]*(.[0-9]*))?").unwrap();

        let groups = match re.captures(input_string) {
            Some(groups) => groups,
            None => panic!("Syntax error for --points-number parameter"),
        };

        NumericFilter {
            operator: groups.name("operator").unwrap().as_str().to_string(),
            header_attribute: attribute_name.to_string(),
            value: groups
                .name("value")
                .unwrap()
                .as_str()
                .parse::<f64>()
                .unwrap(),
        }
    }
}

impl Compare for NumericFilter {
    fn compare(&self, value: &MyHeader) -> bool {
        let operator = self.operator.as_str();

        match self.header_attribute.as_str() {
            "number_of_points" => match operator {
                "=" => self.value == value.number_of_points as f64,
                "!=" => self.value != value.number_of_points as f64,
                ">" => self.value < value.number_of_points as f64,
                "<" => self.value > value.number_of_points as f64,
                "<=" => self.value >= value.number_of_points as f64,
                ">=" => self.value <= value.number_of_points as f64,
                _ => panic!("Operator {} not found", operator),
            },
            "file_source_id" => match operator {
                "=" => self.value == value.file_source_id as f64,
                "!=" => self.value != value.file_source_id as f64,
                ">" => self.value < value.file_source_id as f64,
                "<" => self.value > value.file_source_id as f64,
                "<=" => self.value >= value.file_source_id as f64,
                ">=" => self.value <= value.file_source_id as f64,
                _ => panic!("Operator {} not found", operator),
            },
            _ => false,
        }
    }
}

#[test]
fn test_xx() {
    assert_eq!(
        NumericFilter::new(&"<10.0".to_string(), &"".to_string()),
        NumericFilter {
            operator: "<".to_string(),
            header_attribute: "".to_string(),
            value: 10.0
        }
    );
    assert_eq!(
        NumericFilter::new(&"<=15000".to_string(), &"".to_string()),
        NumericFilter {
            operator: "<=".to_string(),
            header_attribute: "".to_string(),
            value: 15000.0
        }
    )
}
