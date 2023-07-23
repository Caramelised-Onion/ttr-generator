// TODO: Create a struct and store the regexes and errors etc
// TODO: properly propagate errors

use std::io;

use regex::Regex;

pub fn lng_lat_pairs_from_multipolygon(mpoly_wkt: &str) -> Option<Vec<Vec<(f64, f64)>>> {
    extract_mpoly_content(mpoly_wkt)
        .map(|mpoly_content| extract_polys_content(&mpoly_content))
        .map(|poly_content| {
            poly_content
                .iter()
                .map(|pc| extract_lng_lat_pairs(pc))
                .collect::<Vec<_>>()
        })
}

fn extract_mpoly_content(mpoly_wkt: &str) -> Option<String> {
    let re = Regex::new(r"\((.*)\)").unwrap();
    re.captures(mpoly_wkt)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

fn extract_polys_content(mpoly_content: &str) -> Vec<String> {
    let re = Regex::new(r"\(\((.*)\)\)").unwrap();
    re.captures_iter(mpoly_content)
        .flat_map(|captures| captures.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

fn extract_lng_lat_pairs(poly_content: &str) -> Vec<(f64, f64)> {
    poly_content
        .split(", ")
        .flat_map(|p| lng_lat_pair_from_string(p))
        .collect()
}

fn lng_lat_pair_from_string(input_str: &str) -> Result<(f64, f64), io::Error> {
    let parts: Vec<&str> = input_str.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unexpected format for lng lat pair",
        ));
    }

    let lng = parts[0].parse::<f64>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Unexpected format for lng lat pair",
        )
    })?;
    let lat = parts[1].parse::<f64>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Unexpected format for lng lat pair",
        )
    })?;
    Ok((lng, lat))
}

#[cfg(test)]
mod test {
    use crate::wktparse::{extract_mpoly_content, lng_lat_pairs_from_multipolygon};

    #[test]
    fn test_extract_mpoly_content() {
        let mpoly_wkt = "MULTIPOLYGON(()()())";
        assert_eq!(extract_mpoly_content(mpoly_wkt), Some("()()()".to_string()));
    }

    #[test]
    fn test_lng_lat_pairs_from_multipolygon() {
        let mpoly_wkt = "MULTIPOLYGON (((1 1, 1 3, 3 3, 3 1, 1 1)), ((4 3, 6 3, 6 1, 4 1, 4 3)))";
        let res = lng_lat_pairs_from_multipolygon(mpoly_wkt).expect("Error parsing mpoly_wkt");
        println!("{:?}", res);
    }
}
