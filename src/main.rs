mod country;
mod distribution;
mod geoquery;
mod models;
mod plot;

use crate::plot::plot_cities;
use clap::Parser;
use geoquery::get_ttr_cities;
use plot::plot_country_wkt;

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
    // let cities = get_ttr_cities(&client, &args.country, args.num_cities).await;
    let country_outline_wkt = client
        .get_country_outline(args.country)
        .await
        .expect("Could not get ountry outline from API");

    println!("wkt\n{}", country_outline_wkt);

    // plot_cities(&cities);
    plot_country_wkt(country_outline_wkt);
}
