mod country;
mod distribution;
mod geoquery;
mod models;
mod plot;

use clap::Parser;
use country::get_largest_polygon;
use geo_types::Geometry;
use geoquery::get_ttr_cities;
use geozero::ToWkt;
use plot::plot_gameboard;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    country: String,
    #[arg(short, long, default_value_t = 32)]
    num_cities: usize,
}

const BASE_URL: &str = "http://localhost:3000";

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = cities_client::client::Client::new(BASE_URL);
    let country_outline_wkt = client
        .get_country_outline(args.country)
        .await
        .expect("Could not get ountry outline from API");
    let largest_polygon = get_largest_polygon(&country_outline_wkt)
        .expect("Could not get the largest polygon from the country outline wkt");
    let largest_polygon_geom: Geometry<f64> = largest_polygon.clone().into();
    let largest_polygon_wkt = largest_polygon_geom.to_wkt()
        .expect("Could not parse geometry into wkt");

    let cities = get_ttr_cities(&client, &largest_polygon_wkt, args.num_cities).await;

    plot_gameboard(largest_polygon, &cities);
}
