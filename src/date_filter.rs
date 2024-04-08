use crate::filter::Compare;
use crate::header::MyHeader;
use crate::list_filter::ListFilter;
use crate::numeric_filter::NumericFilter;
use chrono::NaiveDate;
use log::{error, log, warn};
use regex::Regex;

//use crate::filter::Compare;
//use crate::header::MyHeader;
#[derive(Debug)]
pub struct DateFilter {
    value: NaiveDate,
    operator: String,
    header_attribute: String,
}

impl DateFilter {
    pub fn new(input_string: &String, date_format: &String, attribute_name: &String) -> DateFilter {
        let re: Regex =
            Regex::new(r"(?P<operator>=|<=|>=|<|>|!=)(?P<date>.*)").unwrap();

        let groups = match re.captures(input_string) {
            Some(groups) => groups,
            None => panic!("Syntax error for --date parameter"),
        };

        let operator = groups.name("operator").unwrap().as_str().to_string();
        let date_string = groups.name("date").unwrap().as_str().to_string();
        let parsed_date =
            NaiveDate::parse_from_str(date_string.as_str(), date_format.as_str())
                .expect("Unable to parse the date with the current formatter");

        DateFilter {
            value: parsed_date,
            operator,
            header_attribute: attribute_name.to_string(),
        }
    }
}

impl Compare for DateFilter {
    fn compare(&self, value: &MyHeader) -> bool {
        let operator = self.operator.as_str();
        match value.date {
            None => {
                warn!("No date in header: Filter ignored");
                true
            }
            Some(date) => match operator {
                "=" => self.value == date,
                "!=" => self.value != date,
                ">" => self.value < date,
                "<" => self.value > date,
                "<=" => self.value >= date,
                ">=" => self.value <= date,
                _ => panic!("Operator {} not found", operator),
            },
        }
    }
}
