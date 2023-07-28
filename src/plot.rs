use plotters::prelude::*;

use crate::models::CitySimple;

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
