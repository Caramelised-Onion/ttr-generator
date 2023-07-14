use cities_common::{models::City, queries::SortOrder};
use clap::Parser;
use plotters::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    country: String,
    #[arg(short, long, default_value_t = 32)]
    num_cities: usize,
}

struct CitySimple {
    name: String,
    lat: f64,
    lng: f64,
}

impl CitySimple {
    fn from_city(city: &City) -> Self {
        CitySimple {
            name: city.name.clone(),
            lat: city.lat,
            lng: city.lng,
        }
    }
}

const BASE_URL: &str = "http://localhost:3000";

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = cities_client::client::Client::new(BASE_URL);

    let cities_to_plot =
        get_most_populated_cities_in_country(&client, &args.country, args.num_cities).await;
    plot_cities(&cities_to_plot);
}

async fn get_most_populated_cities_in_country(
    client: &cities_client::client::Client,
    country_iso: &str,
    num_cities: usize,
) -> Vec<CitySimple> {
    let cities_query = cities_common::queries::CitiesQuery {
        country: Some(country_iso.to_string()),
        sort_by_population: Some(SortOrder::DESC),
        limit: Some(num_cities),
        ..Default::default()
    };
    client
        .get_cities(&cities_query)
        .await
        .expect("failed to call cities api or something")
        .iter()
        .map(|c| CitySimple::from_city(c))
        .collect()
}

fn plot_cities(cities: &Vec<CitySimple>) {
    let drawing_area = BitMapBackend::new("ttr.png", (600, 400)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let min_x = cities
        .iter()
        .min_by_key(|c| (c.lng * 10_000.0) as usize)
        .unwrap()
        .lng;
    let max_x = cities
        .iter()
        .max_by_key(|c| (c.lng * 10_000.0) as usize)
        .unwrap()
        .lng;
    let min_y = cities
        .iter()
        .min_by_key(|c| (c.lat * 10_000.0) as usize)
        .unwrap()
        .lat;
    let max_y = cities
        .iter()
        .max_by_key(|c| (c.lat * 10_000.0) as usize)
        .unwrap()
        .lat;

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)
        .unwrap();

    chart
        .draw_series(
            cities
                .iter()
                .map(|c| {
                    EmptyElement::at((c.lat, c.lng))
                        + Circle::new((0, 0), size, style)
                        + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
                }),
        )
        .expect("Failed to make graphic");

}
