use cities_common::queries::SortOrder;

use crate::models::CitySimple;

pub async fn get_most_populated_cities_in_country(
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
