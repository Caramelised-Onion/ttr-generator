use cities_common::models::City;

#[derive(Debug)]
pub struct CitySimple {
    pub name: String,
    pub lat: f64,
    pub lng: f64,
}

impl CitySimple {
    pub fn from_city(city: &City) -> Self {
        CitySimple {
            name: city.name.clone(),
            lat: city.lat,
            lng: city.lng,
        }
    }
}
