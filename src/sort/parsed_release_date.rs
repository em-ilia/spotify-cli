use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub struct ParsedReleaseDate {
    year: Option<u16>,
    month: Option<u8>,
    day: Option<u8>,
}

impl TryFrom<&str> for ParsedReleaseDate {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut prd: Self = ParsedReleaseDate {
            year: None,
            month: None,
            day: None,
        };

        let split: Vec<&str> = value.split('-').collect();

        if split.is_empty() || split.len() > 3 {
            return Err("Improper number of split groups".to_owned());
        }
        if !split.is_empty() {
            if let Ok(year) = split[0].parse::<u16>() {
                prd.year = Some(year);
            } else {
                return Err("Failed to parse year".to_owned());
            }
        }
        if split.len() >= 2 {
            if let Ok(month) = split[1].parse::<u8>() {
                prd.month = Some(month);
            } else {
                return Err("Failed to parse month".to_owned());
            }
        }
        if split.len() >= 3 {
            if let Ok(day) = split[2].parse::<u8>() {
                prd.day = Some(day);
            } else {
                return Err("Failed to parse day".to_owned());
            }
        }

        Ok(prd)
    }
}

impl PartialOrd for ParsedReleaseDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let year = match (self.year, other.year) {
            (None, None) => None,
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),
            (Some(a), Some(b)) => Some(a.cmp(&b))
        };
        match year {
            None => return None,
            Some(Ordering::Less) => return Some(Ordering::Less),
            Some(Ordering::Greater) => return Some(Ordering::Greater),
            Some(Ordering::Equal) => ()
        }

        // Note that we only fall through to here in the equal case!
        let month = match (self.month, other.month) {
            (None, None) => None,
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),
            (Some(a), Some(b)) => Some(a.cmp(&b))
        };
        match month {
            None => return year,
            Some(Ordering::Less) => return Some(Ordering::Less),
            Some(Ordering::Greater) => return Some(Ordering::Greater),
            Some(Ordering::Equal) => ()
        }

        let day = match (self.day, other.day) {
            (None, None) => None,
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),
            (Some(a), Some(b)) => Some(a.cmp(&b))
        };
        match day {
            None => month,
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_release_date_parse() {
        let str1 = "2012";
        let prd1 = ParsedReleaseDate { year: Some(2012), month: None, day: None };
        assert_eq!(ParsedReleaseDate::try_from(str1), Ok(prd1));

        let str2 = "2012-6";
        let prd2 = ParsedReleaseDate { year: Some(2012), month: Some(6), day: None };
        assert_eq!(ParsedReleaseDate::try_from(str2), Ok(prd2));

        let str3 = "2012-6-21";
        let prd3 = ParsedReleaseDate { year: Some(2012), month: Some(6), day: Some(21) };
        assert_eq!(ParsedReleaseDate::try_from(str3), Ok(prd3));
    }

    #[test]
    fn test_parsed_release_date_partialord() {
        let prd1 = ParsedReleaseDate { year: None, month: None, day: None };
        assert_eq!(prd1.partial_cmp(&prd1), None);

        let prd2 = ParsedReleaseDate { year: Some(2010), month: None, day: None };
        let prd3 = ParsedReleaseDate { year: Some(2011), month: None, day: None };
        assert_eq!(prd2.partial_cmp(&prd1), Some(Ordering::Greater));
        assert_eq!(prd2.partial_cmp(&prd2), Some(Ordering::Equal));
        assert_eq!(prd2.partial_cmp(&prd3), Some(Ordering::Less));

        let prd4 = ParsedReleaseDate { year: Some(2011), month: Some(3), day: None };
        let prd5 = ParsedReleaseDate { year: Some(2011), month: Some(4), day: None };
        assert_eq!(prd4.partial_cmp(&prd1), Some(Ordering::Greater));
        assert_eq!(prd4.partial_cmp(&prd2), Some(Ordering::Greater));
        assert_eq!(prd4.partial_cmp(&prd3), Some(Ordering::Greater));
        assert_eq!(prd4.partial_cmp(&prd4), Some(Ordering::Equal));
        assert_eq!(prd4.partial_cmp(&prd5), Some(Ordering::Less));

        let prd6 = ParsedReleaseDate { year: Some(2011), month: Some(4), day: Some(2) };
        let prd7 = ParsedReleaseDate { year: Some(2011), month: Some(4), day: Some(3) };
        assert_eq!(prd6.partial_cmp(&prd1), Some(Ordering::Greater));
        assert_eq!(prd6.partial_cmp(&prd2), Some(Ordering::Greater));
        assert_eq!(prd6.partial_cmp(&prd3), Some(Ordering::Greater));
        assert_eq!(prd6.partial_cmp(&prd4), Some(Ordering::Greater));
        assert_eq!(prd6.partial_cmp(&prd5), Some(Ordering::Greater));
        assert_eq!(prd6.partial_cmp(&prd6), Some(Ordering::Equal));
        assert_eq!(prd6.partial_cmp(&prd7), Some(Ordering::Less));

        let prd8 = ParsedReleaseDate { year: Some(2001), month: Some(1), day: Some(1) };
        assert_eq!(prd8.partial_cmp(&prd1), Some(Ordering::Greater));
        assert_eq!(prd8.partial_cmp(&prd2), Some(Ordering::Less));
        assert_eq!(prd8.partial_cmp(&prd3), Some(Ordering::Less));
        assert_eq!(prd8.partial_cmp(&prd4), Some(Ordering::Less));
        assert_eq!(prd8.partial_cmp(&prd5), Some(Ordering::Less));
        assert_eq!(prd8.partial_cmp(&prd6), Some(Ordering::Less));
        assert_eq!(prd8.partial_cmp(&prd7), Some(Ordering::Less));
    }
}
