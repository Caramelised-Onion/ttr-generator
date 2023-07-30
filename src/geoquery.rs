use crate::{distribution::filter_out_cities_too_close, models::CitySimple};
use cities_common::queries::SortOrder;
use geo_types::{Geometry, MultiPolygon};
use geozero::ToWkt;

/// 1. get top X cities
/// 2. Init Forbidden Area
/// 3. Add city and add a surroudning area of it to forbidden cities if a buffer around city does not intersect with forbidden areas
pub async fn get_ttr_cities(
    client: &cities_client::client::Client,
    country_iso: &str,
    num_cities: usize,
) -> Vec<CitySimple> {
    let mut cities = vec![];
    let mut forbidden_area: MultiPolygon<f64> = MultiPolygon::new(vec![]);
    while cities.len() < num_cities {
        let unfiltered_cities = get_most_populated_cities_in_country_not_in_forbidden_area(
            client,
            country_iso,
            num_cities - cities.len(),
            forbidden_area.clone(),
        )
        .await;
        let (filtered_cities, added_forbidden_area) =
            filter_out_cities_too_close(unfiltered_cities, 0.5);
        forbidden_area.0.extend(added_forbidden_area);
        cities.extend(filtered_cities);
    }
    cities
}

async fn get_most_populated_cities_in_country_not_in_forbidden_area(
    client: &cities_client::client::Client,
    country_iso: &str,
    num_cities: usize,
    forbidden_area: MultiPolygon<f64>,
) -> Vec<CitySimple> {
    let mut geom_out_q: Option<String> = None;
    if !forbidden_area.0.is_empty() {
        let geom_out: Geometry<f64> = forbidden_area.into();
        let geom_out_wkt = geom_out.to_wkt().expect("Major wkt failure");
        geom_out_q = Some(geom_out_wkt);
    }

    let cities_query = cities_common::queries::CitiesQuery {
        country: Some(country_iso.to_string()),
        geometry_out: geom_out_q,
        sort_by_population: Some(SortOrder::DESC),
        limit: Some(num_cities),
        ..Default::default()
    };
    client
        .get_cities(&cities_query)
        .await
        .expect("failed to call cities api or something")
        .iter()
        .map(CitySimple::from_city)
        .collect()
}
