use geo::Area;
use geo_types::{Geometry, MultiPolygon, Polygon};
use geozero::{wkt::WktStr, ToGeo};

/// Takes the wkt representation of a country and returns the "main" area as a polygon
/// TODO: move this functionality to the db
pub fn get_largest_polygon(country_wkt: &str) -> Option<Polygon<f64>> {
    let parsed_geom = WktStr(country_wkt).to_geo();
    let multi_polygon = match parsed_geom {
        Ok(Geometry::MultiPolygon(mp)) => mp,
        Ok(_) => {
            panic!("Expected a MultiPolygon");
        }
        Err(e) => {
            panic!("Error parsing WKT: {}", e);
        }
    };
    find_largest_polygon(multi_polygon)
}

fn find_largest_polygon(mp: MultiPolygon<f64>) -> Option<Polygon<f64>> {
    mp.iter()
        .max_by_key(|p| (p.unsigned_area() * 10_000.0) as isize)
        .cloned()
}
