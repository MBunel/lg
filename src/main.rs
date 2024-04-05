mod date_filter;
mod filter;
mod header;
mod list_filter;
mod numeric_filter;
mod spatial_filter;
mod string_filter;

use chrono::NaiveDate;
use clap::{Arg, ArgAction, ArgGroup, Command};
use std::default::Default;

use wkt::TryFromWkt;

use crate::filter::{Filter, LasHeaderFilter};
use crate::list_filter::ListFilter;
use crate::numeric_filter::NumericFilter;
use crate::string_filter::StringFilter;

use crate::spatial_filter::{SpatialFilter, SpatialPredicate};
use geo::Geometry;
use las::{Read, Reader};
use walkdir::{DirEntry, WalkDir};

fn file_filter(entry: DirEntry, inverse: bool, extensions: &Vec<&str>, filter: &LasHeaderFilter) {
    let extension = entry.path().extension().unwrap().to_ascii_lowercase();
    let extension_str = extension.to_str().unwrap();

    if extensions.contains(&extension_str) {
        let path = entry.path();
        let reader = Reader::from_path(path);

        match reader {
            Ok(reader) => {
                let header = header::MyHeader::new(reader.header());

                match inverse ^ filter.filter(&header) {
                    true => {
                        println!("'{}'", entry.path().display())
                    }
                    false => {}
                }
            }
            Err(error) => {
                let path_string = path.display();
                eprintln!("Impossible to read {path_string}: {error}")
            }
        }
    };
}

// Add ignore dir
fn folder_walk(
    folder: &str,
    recursive: bool,
    f_links: bool,
    inverse: bool,
    // ignore_dirs: &Vec<&str>,
    extensions: &Vec<&str>,
    filter: &LasHeaderFilter,
) {
    let folder_iterator = WalkDir::new(folder)
        .max_depth(if recursive { usize::MAX } else { 1 })
        .follow_links(f_links);

    for entry in folder_iterator {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    file_filter(entry, inverse, extensions, filter);
                }
            }
        };
    }
}

fn main() {
    let args = Command::new("lg")
        .version("0.1.2")
        .author("Mattia B. <mattia.bunel@ign.fr>")
        .about(
            "lg (LasGrep) is a tool to filter asprs's las and laz files, with the informations \
        contained in the file header.",
        )
        // Arguments Needed
        .arg(
            Arg::new("input")
                .required(true)
                .action(ArgAction::Append)
                .help("help string"),
        )
        // Options
        .next_help_heading("Find name")
        .arg(
            Arg::new("las_version")
                //.short('t')
                .long("las-version")
                .long("las-version")
                .value_parser(clap::value_parser!(String))
                .help("Selects files according to the version of the las standard.")
                .long_help(
                    "Selects files according to the version of the las standard. The \
                selection can be made using the semver syntax.\n\n
                The following operators are supported :\n\n
                   - \"=\" : When a version is \n
                   - \"<\" \n
                   - \">\" \n
                   - \"~\" \n
                   - \"^\" \n
                   - \"<=\" \n
                   - \">=\"
                ",
                ),
        )
        .arg(
            Arg::new("generating_software")
                .long("generating-software")
                .value_parser(clap::value_parser!(String))
                .help("Selects files according the generating software"),
        )
        .arg(
            Arg::new("system_identifier")
                .long("system-identifier")
                .value_parser(clap::value_parser!(String))
                .help("Selects files according the system identifier"),
        )
        .arg(
            Arg::new("date")
                .long("date")
                .value_parser(clap::value_parser!(NaiveDate))
                .help("Not implemented now"),
        )
        .arg(
            Arg::new("guid")
                .long("guid")
                .help("Selects files according the guid"),
        )
        .arg(
            Arg::new("file_source_id")
                .long("file-source-id")
                .help("Selects files according the file source id"),
        )
        .arg(
            Arg::new("gps_time_type")
                .long("gps-time-type")
                .value_parser(["week", "standard"])
                .help("Selects files according the type of gps time"),
        )
        .arg(
            Arg::new("point_format")
                .long("point-format")
                .value_parser(clap::value_parser!(u8).range(0..10))
                .value_delimiter(',')
                .help("Selects files according the format of points (eg. \"1\", \"2,6,10\")"),
        )
        .arg(
            Arg::new("transform")
                .long("transform")
                .help("Not implemented now"),
        )
        .arg(Arg::new("points_number").long("points-number").help(
            "Selects files according the number of points (eg. \">1000\"). \
                Allowed operators : \"=\", \"!=\", \"<=\", \">=\", \"<\" and \">\".",
        ))
        // Spatial filters
        .next_help_heading("Spatial filters")
        .arg(
            Arg::new("wkt")
                .long("wkt")
                .value_parser(clap::value_parser!(String))
                .help("Todo"),
        )
        .arg(
            Arg::new("intersects")
                .long("intersects")
                .action(ArgAction::SetTrue)
                .help("Todo"),
        )
        .arg(
            Arg::new("within")
                .long("within")
                .action(ArgAction::SetTrue)
                .help("Todo"),
        )
        .arg(
            Arg::new("centroid_within")
                .long("centroid-within")
                .action(ArgAction::SetTrue)
                .help("Todo"),
        )
        .arg(
            Arg::new("contains")
                .long("contains")
                .action(ArgAction::SetTrue)
                .help("Todo"),
        )
        .arg(
            Arg::new("distance")
                .long("distance")
                .help("Todo (Distance in CRS's units)"),
        )
        .next_help_heading("Find name 2")
        .arg(
            Arg::new("extensions")
                .long("extensions")
                .default_values(["las", "laz"])
                .value_parser(["las", "laz"])
                .value_delimiter(',')
                .help("Extensions allowed"),
        )
        .arg(
            Arg::new("exclude-dirs")
                .long("exclude-dirs")
                .action(ArgAction::Append)
                .help("Not implemented now"),
        )
        .arg(
            Arg::new("invert")
                .long("invert")
                .short('i')
                .action(ArgAction::SetTrue)
                .help("Invert the selection"),
        )
        .arg(
            Arg::new("recursive")
                .long("recursive")
                .short('R')
                .action(ArgAction::SetTrue)
                .help("List files recursively"),
        )
        .arg(
            Arg::new("follow_links")
                .long("follow-links")
                .short('L')
                .action(ArgAction::SetTrue)
                .help("Follow the symbolics links"),
        )
        .arg(
            Arg::new("canonicalize")
                .short('c')
                .long("canonicalize")
                .action(ArgAction::SetTrue)
                .help("Not implemented now"),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .action(ArgAction::SetTrue)
                .help("Print the parameters and quit (for debug purposes)"),
        )
        //.group(ArgGroup::new("Test").arg("transform").arg("extensions"))
        // Groups
        .group(
            ArgGroup::new("spatial_predicates")
                .arg("within")
                .arg("contains")
                .arg("intersects")
                .arg("centroid_within")
                .arg("contains")
                .arg("distance")
                .requires("spatial_readers")
                .multiple(false),
        )
        .group(
            ArgGroup::new("spatial_readers")
                .arg("wkt")
                .requires("spatial_predicates"),
        )
        .get_matches();

    // Creation of filter object
    let mut filter = LasHeaderFilter {
        ..Default::default()
    };

    // Numeric Type
    if let Some(point_number) = args.get_one("points_number") {
        filter.number_of_points_filter = Some(Filter::FNumeric(NumericFilter::new(
            point_number,
            &String::from("number_of_points"),
        )));
    }

    if let Some(file_source_id) = args.get_one("file_source_id") {
        filter.file_source_id_filter = Some(Filter::FNumeric(NumericFilter::new(
            file_source_id,
            &String::from("file_source_id"),
        )));
    }

    if let Some(point_format) = args.get_many::<u8>("point_format") {
        filter.point_format_filter = Some(Filter::FList(ListFilter::new(
            point_format.map(|x| x.clone()).collect::<Vec<_>>(),
            &String::from("file_source_id"),
        )));
    }

    // String Type
    if let Some(gps_time_type) = args.get_one("gps_time_type") {
        filter.gps_time_type_filter = Some(Filter::FString(StringFilter::new(
            gps_time_type,
            &String::from("gps_time_type"),
        )));
    }

    if let Some(las_version) = args.get_one("las_version") {
        filter.version_filter = Some(Filter::FString(StringFilter::new(
            las_version,
            &String::from("las_version"),
        )));
    }

    if let Some(guid) = args.get_one("guid") {
        filter.guid_filter = Some(Filter::FString(StringFilter::new(
            guid,
            &String::from("guid"),
        )))
    }

    if let Some(generating_software) = args.get_one("generating_software") {
        filter.generating_software_filter = Some(Filter::FString(StringFilter::new(
            generating_software,
            &String::from("generating_software"),
        )))
    }

    if let Some(system_identifier) = args.get_one("system_identifier") {
        filter.system_identifier_filter = Some(Filter::FString(StringFilter::new(
            system_identifier,
            &String::from("system_identifier"),
        )))
    }

    if let Some(wkt_string) = args.get_one::<String>("wkt") {
        let geometry: Geometry<f64> = Geometry::try_from_wkt_str(wkt_string).unwrap();

        let mut predicate: SpatialPredicate = SpatialPredicate::Intersects;

        if args.get_flag("intersects") {
            predicate = SpatialPredicate::Intersects
        } else if args.get_flag("within") {
            predicate = SpatialPredicate::Within
        } else if args.get_flag("contains") {
            predicate = SpatialPredicate::Contains
        } else if args.get_flag("centroid_within") {
            predicate = SpatialPredicate::CentroidWithin
        } else {
            if let Some(distance) = args.get_one::<String>("distance") {
                predicate = SpatialPredicate::CentroidDistance {
                    dist: distance.parse::<f64>().unwrap(),
                }
            }
        }
        filter.geom_filter = Some(Filter::FSpatial(SpatialFilter::new(geometry, predicate)))
    }

    // Extraction of path
    let paths = args
        .get_many::<String>("input")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let extensions = args
        .get_many::<String>("extensions")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    match args.get_flag("debug") {
        true => {
            println!("Paths: {:?}", &paths);
            println!("Extensions: {:?}", &extensions);
            println!("Filters: {:?}", &filter);
        }
        false => {
            // Main code
            for path in paths {
                folder_walk(
                    path,
                    args.get_flag("recursive"),
                    args.get_flag("follow_links"),
                    args.get_flag("invert"),
                    &extensions,
                    &filter,
                )
            }
        }
    }
}
