use crate::filter::Compare;
use crate::header::MyHeader;

use geo::Centroid;
use geo::{Contains, EuclideanDistance, Intersects, Within};

#[derive(Debug)]
pub enum SpatialPredicate {
    Intersects,
    Within,
    CentroidWithin,
    Contains,
    CentroidDistance { dist: f64 },
}

#[derive(Debug)]
pub struct SpatialFilter {
    predicate: SpatialPredicate,
    geom: geo::Geometry,
}

impl SpatialFilter {
    pub fn new(geom: geo::Geometry, predicate: SpatialPredicate) -> SpatialFilter {
        SpatialFilter { predicate, geom }
    }
}

impl Compare for SpatialFilter {
    fn compare(&self, value: &MyHeader) -> bool {
        match self.predicate {
            SpatialPredicate::Intersects => value.bbox.intersects(&self.geom),
            SpatialPredicate::Contains => value.bbox.contains(&self.geom),
            SpatialPredicate::Within => value.bbox.is_within(&self.geom),
            SpatialPredicate::CentroidWithin => value.centroid.is_within(&self.geom),
            SpatialPredicate::CentroidDistance { dist } => {
                value
                    .centroid
                    .euclidean_distance(&self.geom.centroid().unwrap())
                    <= dist
            }
        }
    }
}
