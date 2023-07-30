use geo::CoordsIter;
use geo_types::Geometry;
use geozero::ToWkt;
use plotters::prelude::*;

use crate::{country::get_largest_polygon, models::CitySimple};

pub fn plot_cities(cities: &[CitySimple]) {
    let drawing_area = BitMapBackend::new("ttr.png", (1500, 900)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let min_x = cities
        .iter()
        .min_by_key(|c| (c.lng * 10_000.0) as isize)
        .unwrap()
        .lng;
    let max_x = cities
        .iter()
        .max_by_key(|c| (c.lng * 10_000.0) as isize)
        .unwrap()
        .lng;
    let min_y = cities
        .iter()
        .min_by_key(|c| (c.lat * 10_000.0) as isize)
        .unwrap()
        .lat;
    let max_y = cities
        .iter()
        .max_by_key(|c| (c.lat * 10_000.0) as isize)
        .unwrap()
        .lat;

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)
        .unwrap();

    chart
        .draw_series(cities.iter().map(|c| {
            EmptyElement::at((c.lng, c.lat))
                + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
                + Text::new(c.name.to_string(), (0, 15), ("sans-serif", 15))
        }))
        .expect("Failed to make graphic");
}

pub fn plot_country_wkt(country_wkt: String) {
    let largest_polygon = get_largest_polygon(&country_wkt)
        .expect("Could not find the largest polygon from this wkt representation");

    let largest_polygon_geom: Geometry<f64> = largest_polygon.clone().into();
    println!("so-called largest polygon\n{}", largest_polygon_geom.to_wkt().unwrap());

    let min_x = largest_polygon.coords_iter()
        .min_by_key(|c| (c.x * 10_000.0) as isize)
        .map(|c| c.x)
        .unwrap();

    let max_x = largest_polygon.coords_iter()
        .max_by_key(|c| (c.x * 10_000.0) as isize)
        .map(|c| c.x)
        .unwrap();

    let min_y = largest_polygon.coords_iter()
        .min_by_key(|c| (c.y * 10_000.0) as isize)
        .map(|c| c.y)
        .unwrap();

    let max_y = largest_polygon.coords_iter()
        .max_by_key(|c| (c.y * 10_000.0) as isize)
        .map(|c| c.y)
        .unwrap();

    let aspect_ratio = ((max_x - min_x) / (max_y - min_y)).abs();
    let height= 900;
    let width = (height as f64 * aspect_ratio) as u32;

    let drawing_area = BitMapBackend::new("ttr.png", (width, height)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)
        .unwrap();

    chart
        .draw_series(LineSeries::new(largest_polygon.exterior().coords_iter().map(|c| (c.x, c.y)), &RED))
        .expect("Failed to make graphic");

    // Next:
    // 1. Get Vector of pairs
    // 2. Get min and max x and y values
    // 3. Calculate Aspect Ratio
    // 4. Use Aspect Ratio to determine chart size and plot the outline
}
