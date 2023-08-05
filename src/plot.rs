use geo::CoordsIter;
use plotters::prelude::*;
use geo_types::Polygon;

use crate::models::CitySimple;

// TODO make this take largest_polygon as an arg
pub fn plot_gameboard(largest_polygon: Polygon<f64>, cities: &[CitySimple]) {
    let min_x = largest_polygon
        .coords_iter()
        .min_by_key(|c| (c.x * 10_000.0) as isize)
        .map(|c| c.x)
        .unwrap();

    let max_x = largest_polygon
        .coords_iter()
        .max_by_key(|c| (c.x * 10_000.0) as isize)
        .map(|c| c.x)
        .unwrap();

    let min_y = largest_polygon
        .coords_iter()
        .min_by_key(|c| (c.y * 10_000.0) as isize)
        .map(|c| c.y)
        .unwrap();

    let max_y = largest_polygon
        .coords_iter()
        .max_by_key(|c| (c.y * 10_000.0) as isize)
        .map(|c| c.y)
        .unwrap();

    let aspect_ratio = ((max_x - min_x) / (max_y - min_y)).abs();
    let height = 900;
    let width = (height as f64 * aspect_ratio) as u32;

    let drawing_area = BitMapBackend::new("ttr.png", (width, height)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            largest_polygon.exterior().coords_iter().map(|c| (c.x, c.y)),
            &RED,
        ))
        .expect("Failed to make graphic");

    chart
        .draw_series(cities.iter().map(|c| {
            EmptyElement::at((c.lng, c.lat))
                + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
                + Text::new(c.name.to_string(), (0, 15), ("sans-serif", 15))
        }))
        .expect("Failed to make graphic");
}
