use chrono::{Date, Utc};
use geo::{Centroid, LineString, Polygon};

use las::Bounds;
use semver::Version;
use uuid::Uuid;

pub struct MyHeader {
    pub file_source_id: u16,
    pub gps_time_type: las::GpsTimeType,
    pub has_synthetic_return_numbers: bool,
    pub guid: Uuid,
    pub version: Version,
    pub system_identifier: String,
    pub generating_software: String,
    pub date: Option<Date<Utc>>,
    pub point_format: las::point::Format,
    //tranform
    pub bbox: Polygon,
    pub centroid: geo::Point,
    pub number_of_points: u64,
}

impl MyHeader {
    fn bounds2poly(las_bounds: Bounds) -> Polygon {
        let poly: Polygon = Polygon::new(
            LineString::from(vec![
                (las_bounds.min.x, las_bounds.min.y),
                (las_bounds.min.x, las_bounds.max.y),
                (las_bounds.max.x, las_bounds.max.y),
                (las_bounds.max.x, las_bounds.min.y),
                (las_bounds.min.x, las_bounds.min.y),
            ]),
            vec![],
        );

        return poly;
    }

    pub fn new(las_header: &las::Header) -> MyHeader {
        let bbox_poly = Self::bounds2poly(las_header.bounds());

        //println!("{:?}", las_header.point_format().to_u8());

        MyHeader {
            file_source_id: las_header.file_source_id(),
            gps_time_type: las_header.gps_time_type(),
            has_synthetic_return_numbers: las_header.has_synthetic_return_numbers(),
            guid: las_header.guid(),
            version: Version::new(
                las_header.version().major as u64,
                las_header.version().minor as u64,
                0_u64,
            ),
            system_identifier: las_header.system_identifier().to_string(),
            generating_software: las_header.generating_software().to_string(),
            date: las_header.date(),
            point_format: las_header.point_format().clone(),
            //transform
            bbox: bbox_poly.clone(),
            centroid: bbox_poly.centroid().unwrap(),
            number_of_points: las_header.number_of_points(),
        }
    }
}
