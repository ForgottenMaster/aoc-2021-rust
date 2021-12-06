use {
    super::{parse_line_error::ParseLineError, point::Point},
    std::str::FromStr,
};

/// Struct holding a pair of points indicating the two end points of a line
#[derive(Debug, PartialEq)]
struct Line(Point, Point);

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.trim();
        let mut splits = string
            .split("->")
            .map(|point| point.trim().parse::<Point>());
        if let Some(p1) = splits.next() {
            if let Some(p2) = splits.next() {
                if let None = splits.next() {
                    Ok(Self(p1?, p2?))
                } else {
                    Err(ParseLineError::TooManyParts)
                }
            } else {
                Err(ParseLineError::NotEnoughParts)
            }
        } else {
            Err(ParseLineError::NotEnoughParts)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_from_str_parse_point_error() {
        assert!(match "2i -> 29".parse::<Line>() {
            Err(ParseLineError::ParsePointError(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn test_line_from_str_not_enough_parts() {
        assert!(match "21,82".parse::<Line>() {
            Err(ParseLineError::NotEnoughParts) => true,
            _ => false,
        });
    }

    #[test]
    fn test_line_from_str_too_many_parts() {
        assert!(match "21,82 -> 24,17 -> 74,19".parse::<Line>() {
            Err(ParseLineError::TooManyParts) => true,
            _ => false,
        });
    }

    #[test]
    fn test_line_from_str_success() {
        assert_eq!(
            "21,82 -> 24,17".parse::<Line>().unwrap(),
            Line("21,82".parse().unwrap(), "24,17".parse().unwrap())
        );
    }
}
