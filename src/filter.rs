use std::fmt::{Display, Formatter};
use std::ops::Index;

use crate::date_filter::DateFilter;
use crate::header::MyHeader;
use crate::list_filter::ListFilter;
use crate::numeric_filter::NumericFilter;
use crate::spatial_filter::SpatialFilter;
use crate::string_filter::StringFilter;

pub trait Compare {
    fn compare(&self, value: &MyHeader) -> bool;
}

impl Compare for Filter {
    fn compare(&self, value: &MyHeader) -> bool {
        match self {
            Filter::FNumeric(filter) => filter.compare(value),
            Filter::FSpatial(filter) => filter.compare(value),
            Filter::FString(filter) => filter.compare(value),
            Filter::FDate(filter) => filter.compare(value),
            Filter::FList(filter) => filter.compare(value),
        }
    }
}

#[derive(Debug)]
pub enum Filter {
    FSpatial(SpatialFilter),
    FString(StringFilter),
    FNumeric(NumericFilter),
    FDate(DateFilter),
    FList(ListFilter),
}

pub struct LasHeaderFilterIterator<'a> {
    index: usize,
    max_index: usize,
    las_header_filter: &'a LasHeaderFilter,
}

#[derive(Default, Debug)]
pub struct LasHeaderFilter {
    pub file_source_id_filter: Option<Filter>,
    pub gps_time_type_filter: Option<Filter>,
    pub synthetic_return_numbers_filter: Option<Filter>,
    pub guid_filter: Option<Filter>,
    pub version_filter: Option<Filter>,
    pub system_identifier_filter: Option<Filter>,
    pub generating_software_filter: Option<Filter>,
    pub date_filter: Option<Filter>,
    pub point_format_filter: Option<Filter>,
    pub transform_filter: Option<Filter>,
    pub geom_filter: Option<Filter>,
    pub number_of_points_filter: Option<Filter>,
}

impl LasHeaderFilter {
    pub fn new() -> LasHeaderFilter {
        LasHeaderFilter {
            file_source_id_filter: None,
            gps_time_type_filter: None,
            synthetic_return_numbers_filter: None,
            guid_filter: None,
            version_filter: None,
            system_identifier_filter: None,
            generating_software_filter: None,
            date_filter: None,
            point_format_filter: None,
            transform_filter: None,
            geom_filter: None,
            number_of_points_filter: None,
        }
    }
    pub fn filter(&self, header: &MyHeader) -> bool {
        for i in &mut self.into_iter() {
            match i {
                Some(filter) => match filter.compare(header) {
                    true => {}
                    false => return false,
                },
                None => {}
            }
        }
        return true;
    }
}

impl Index<usize> for LasHeaderFilter {
    type Output = Option<Filter>;

    fn index(&self, index: usize) -> &Option<Filter> {
        match index {
            0 => &self.file_source_id_filter,
            1 => &self.gps_time_type_filter,
            2 => &self.synthetic_return_numbers_filter,
            3 => &self.guid_filter,
            4 => &self.version_filter,
            5 => &self.system_identifier_filter,
            6 => &self.generating_software_filter,
            7 => &self.date_filter,
            8 => &self.point_format_filter,
            9 => &self.transform_filter,
            10 => &self.geom_filter,
            11 => &self.number_of_points_filter,
            _ => &None,
        }
    }
}

impl<'a> IntoIterator for &'a LasHeaderFilter {
    type Item = &'a Option<Filter>;
    type IntoIter = LasHeaderFilterIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LasHeaderFilterIterator {
            index: 0,
            max_index: 11,
            las_header_filter: self,
        }
    }
}

impl<'a> Iterator for LasHeaderFilterIterator<'a> {
    type Item = &'a Option<Filter>;

    fn next(&mut self) -> Option<&'a Option<Filter>> {
        if self.index <= self.max_index {
            let result = &self.las_header_filter[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl Display for LasHeaderFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}