use std::f64::consts::PI;

use geo::Intersects;
use geo_types::{coord, polygon, Polygon, MultiPolygon, Point, LineString, Coord};

use crate::models::CitySimple;

fn create_octagon(center: Point<f64>, deg_distance: f64) -> Polygon<f64> {
    let angle_increment = 2.0 * PI / 8.0;

    let vertices: Vec<_> = (0..8)
        .map(|i| {
            let angle = angle_increment * i as f64;
            let x = center.x() + deg_distance * angle.cos();
            let y = center.y() + deg_distance * angle.sin();
            coord!{x: x, y: y}
        })
        .collect();

    let line_string = LineString(vertices);
    Polygon::new(line_string, vec![])
}

// 1. get top X cities
// 2. Init Forbidden Area
// 3. Add city and add a surroudning area of it to forbidden cities if a buffer around city does not intersect with forbidden areas



pub fn filter_out_cities_too_close(cities: Vec<CitySimple>, deg_threshold: f64) -> Vec<CitySimple> {
    let mut res = vec![];
    let mut forbidden_area: MultiPolygon<f64> = MultiPolygon::new(vec![]);

    for city in cities {

        let point: Point = (city.lng, city.lat).into();

        if !point.intersects(&forbidden_area) {
            // octogon FIGHTTTT!!
            let geom = create_octagon(point, deg_threshold);

            if !geom.intersects(&forbidden_area) {
                res.push(city);
                forbidden_area.0.push(geom);
            }
        }
    }
    res
}

