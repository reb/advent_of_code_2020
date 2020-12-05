/// --- Day 4: Passport Processing ---
///
/// You arrive at the airport only to realize that you grabbed your North Pole
/// Credentials instead of your passport. While these documents are extremely
/// similar, North Pole Credentials aren't issued by a country and therefore
/// aren't actually valid documentation for travel in most of the world.
///
/// It seems like you're not the only one having problems, though; a very long
/// line has formed for the automatic passport scanners, and the delay could
/// upset your travel itinerary.
///
/// Due to some questionable network security, you realize you might be able to
/// solve both of these problems at the same time.
///
/// The automatic passport scanners are slow because they're having trouble
/// detecting which passports have all required fields. The expected fields are
/// as follows:
///
///     byr (Birth Year)
///     iyr (Issue Year)
///     eyr (Expiration Year)
///     hgt (Height)
///     hcl (Hair Color)
///     ecl (Eye Color)
///     pid (Passport ID)
///     cid (Country ID)
///
/// Passport data is validated in batch files (your puzzle input). Each passport
/// is represented as a sequence of key:value pairs separated by spaces or
/// newlines. Passports are separated by blank lines.
///
/// Here is an example batch file containing four passports:
///
/// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
/// byr:1937 iyr:2017 cid:147 hgt:183cm
///
/// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
/// hcl:#cfa07d byr:1929
///
/// hcl:#ae17e1 iyr:2013
/// eyr:2024
/// ecl:brn pid:760753108 byr:1931
/// hgt:179cm
///
/// hcl:#cfa07d eyr:2025 pid:166559648
/// iyr:2011 ecl:brn hgt:59in
///
/// The first passport is valid - all eight fields are present. The second
/// passport is invalid - it is missing hgt (the Height field).
///
/// The third passport is interesting; the only missing field is cid, so it
/// looks like data from North Pole Credentials, not a passport at all! Surely,
/// nobody would mind if you made the system temporarily ignore missing cid
/// fields. Treat this "passport" as valid.
///
/// The fourth passport is missing two fields, cid and byr. Missing cid is fine,
/// but missing any other field is not, so this passport is invalid.
///
/// According to the above rules, your improved system would report 2 valid
/// passports.
///
/// Count the number of valid passports - those that have all required fields.
/// Treat cid as optional. In your batch file, how many passports are valid?
///
/// --- Part Two ---
///
/// The line is moving more quickly now, but you overhear airport security
/// talking about how passports with invalid data are getting through. Better
/// add some data validation, quick!
///
/// You can continue to ignore the cid field, but each other field has strict
/// rules about what values are valid for automatic validation:
///
///     byr (Birth Year) - four digits; at least 1920 and at most 2002.
///     iyr (Issue Year) - four digits; at least 2010 and at most 2020.
///     eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
///     hgt (Height) - a number followed by either cm or in:
///         If cm, the number must be at least 150 and at most 193.
///         If in, the number must be at least 59 and at most 76.
///     hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
///     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
///     pid (Passport ID) - a nine-digit number, including leading zeroes.
///     cid (Country ID) - ignored, missing or not.
///
/// Your job is to count the passports where all required fields are both
/// present and valid according to the above rules. Here are some example
/// values:
///
/// byr valid:   2002
/// byr invalid: 2003
///
/// hgt valid:   60in
/// hgt valid:   190cm
/// hgt invalid: 190in
/// hgt invalid: 190
///
/// hcl valid:   #123abc
/// hcl invalid: #123abz
/// hcl invalid: 123abc
///
/// ecl valid:   brn
/// ecl invalid: wat
///
/// pid valid:   000000001
/// pid invalid: 0123456789
///
/// Here are some invalid passports:
///
/// eyr:1972 cid:100
/// hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
///
/// iyr:2019
/// hcl:#602927 eyr:1967 hgt:170cm
/// ecl:grn pid:012533040 byr:1946
///
/// hcl:dab227 iyr:2012
/// ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
///
/// hgt:59cm ecl:zzz
/// eyr:2038 hcl:74454a iyr:2023
/// pid:3556412378 byr:2007
///
/// Here are some valid passports:
///
/// pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
/// hcl:#623a2f
///
/// eyr:2029 ecl:blu cid:129 byr:1989
/// iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
///
/// hcl:#888785
/// hgt:164cm byr:2001 iyr:2015 cid:88
/// pid:545766238 ecl:hzl
/// eyr:2022
///
/// iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
///
/// Count the number of valid passports - those that have all required fields
/// and valid values. Continue to treat cid as optional. In your batch file, how
/// many passports are valid?
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::slice::Iter;

const INPUT: &str = include_str!("../input/day_04.txt");

pub fn run() {
    let passports = parse_passports(INPUT);

    let valid_passports = passports
        .iter()
        .filter(|passport| valid_passport(passport, false))
        .count();
    println!(
        "With the Country ID as an optional field, the number of valid passports is: {}",
        valid_passports
    );

    let data_validated_passports = passports
        .iter()
        .filter(|passport| valid_passport(passport, true))
        .count();
    println!(
        "With the stricter data validation, the number of valid passports is: {}",
        data_validated_passports
    )
}

type Passport = HashMap<Field, String>;
type ValidationFn = fn(&String) -> bool;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Field {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

impl Field {
    fn from_code(code: String) -> Field {
        ///     byr (Birth Year)
        ///     iyr (Issue Year)
        ///     eyr (Expiration Year)
        ///     hgt (Height)
        ///     hcl (Hair Color)
        ///     ecl (Eye Color)
        ///     pid (Passport ID)
        ///     cid (Country ID)
        match code.as_str() {
            "byr" => Field::BirthYear,
            "iyr" => Field::IssueYear,
            "eyr" => Field::ExpirationYear,
            "hgt" => Field::Height,
            "hcl" => Field::HairColor,
            "ecl" => Field::EyeColor,
            "pid" => Field::PassportID,
            "cid" => Field::CountryID,
            _ => panic!("Unknown code found"),
        }
    }

    fn required_validations() -> Iter<'static, (Field, ValidationFn)> {
        lazy_static! {
            static ref REQUIRED_FIELDS: Vec<(Field, ValidationFn)> = vec![
                (Field::BirthYear, valid_birth_year),
                (Field::IssueYear, valid_issue_year),
                (Field::ExpirationYear, valid_expiration_year),
                (Field::Height, valid_height),
                (Field::HairColor, valid_hair_color),
                (Field::EyeColor, valid_eye_color),
                (Field::PassportID, valid_passport_id),
            ];
        }
        REQUIRED_FIELDS.iter()
    }
}

fn valid_passport(passport: &Passport, validate_data: bool) -> bool {
    Field::required_validations().all(|(field, validate)| match validate_data {
        true => passport.get(&field).map_or(false, |value| validate(value)),
        false => passport.contains_key(&field),
    })
}

fn valid_year(year: &String, min: u16, max: u16) -> bool {
    let parsed: u16 = year.parse().expect("Year was not a number");
    parsed >= min && parsed <= max
}

fn valid_birth_year(year: &String) -> bool {
    ///     byr (Birth Year) - four digits; at least 1920 and at most 2002.
    valid_year(year, 1920, 2002)
}

fn valid_issue_year(year: &String) -> bool {
    ///     iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    valid_year(year, 2010, 2020)
}

fn valid_expiration_year(year: &String) -> bool {
    ///     eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    valid_year(year, 2020, 2030)
}

fn valid_height(height: &String) -> bool {
    ///     hgt (Height) - a number followed by either cm or in:
    ///         If cm, the number must be at least 150 and at most 193.
    ///         If in, the number must be at least 59 and at most 76.

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([0-9]+)(in|cm)$").unwrap();
    }
    let captures = RE.captures(height.as_str());
    match captures {
        Some(groups) => match (groups.get(1), groups.get(2)) {
            (Some(number_match), Some(unit_match)) => {
                match (number_match.as_str().parse::<u16>(), unit_match.as_str()) {
                    (Ok(number), "cm") => number >= 150 && number <= 193,
                    (Ok(number), "in") => number >= 59 && number <= 76,
                    _ => false,
                }
            }
            _ => false,
        },

        _ => false,
    }
}

fn valid_hair_color(hair_color: &String) -> bool {
    ///     hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(hair_color.as_str())
}

fn valid_eye_color(eye_color: &String) -> bool {
    ///     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    lazy_static! {
        static ref POSSIBLE_EYE_COLORS: HashSet<String> =
            vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .into_iter()
                .map(|color| String::from(color))
                .collect();
    }
    POSSIBLE_EYE_COLORS.contains(eye_color)
}

fn valid_passport_id(passport_id: &String) -> bool {
    ///     pid (Passport ID) - a nine-digit number, including leading zeroes.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    RE.is_match(passport_id.as_str())
}

fn parse_passports(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|block| convert_to_passport(block))
        .collect()
}

fn convert_to_passport(block: &str) -> Passport {
    let mut passport = Passport::new();
    for entry in block.split_whitespace() {
        let mut chars = entry.chars();
        // take the first three characters as the code for the field
        let code = chars.by_ref().take(3).collect();
        let field = Field::from_code(code);
        // assert that the next character is a colon (:)
        assert_eq!(chars.next(), Some(':'));
        // the rest is the value of the field
        let value = chars.collect();
        passport.insert(field, value);
    }
    passport
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_passports() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
            byr:1937 iyr:2017 cid:147 hgt:183cm\n\
            \n\
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
            hcl:#cfa07d byr:1929\n\
            \n\
            hcl:#ae17e1 iyr:2013\n\
            eyr:2024\n\
            ecl:brn pid:760753108 byr:1931\n\
            hgt:179cm";

        let mut p1 = Passport::new();
        p1.insert(Field::EyeColor, String::from("gry"));
        p1.insert(Field::PassportID, String::from("860033327"));
        p1.insert(Field::ExpirationYear, String::from("2020"));
        p1.insert(Field::HairColor, String::from("#fffffd"));
        p1.insert(Field::BirthYear, String::from("1937"));
        p1.insert(Field::IssueYear, String::from("2017"));
        p1.insert(Field::CountryID, String::from("147"));
        p1.insert(Field::Height, String::from("183cm"));

        let mut p2 = Passport::new();
        p2.insert(Field::IssueYear, String::from("2013"));
        p2.insert(Field::EyeColor, String::from("amb"));
        p2.insert(Field::CountryID, String::from("350"));
        p2.insert(Field::ExpirationYear, String::from("2023"));
        p2.insert(Field::PassportID, String::from("028048884"));
        p2.insert(Field::HairColor, String::from("#cfa07d"));
        p2.insert(Field::BirthYear, String::from("1929"));

        let mut p3 = Passport::new();
        p3.insert(Field::HairColor, String::from("#ae17e1"));
        p3.insert(Field::IssueYear, String::from("2013"));
        p3.insert(Field::ExpirationYear, String::from("2024"));
        p3.insert(Field::EyeColor, String::from("brn"));
        p3.insert(Field::PassportID, String::from("760753108"));
        p3.insert(Field::BirthYear, String::from("1931"));
        p3.insert(Field::Height, String::from("179cm"));

        let expected = vec![p1, p2, p3];

        assert_eq!(parse_passports(input), expected);
    }

    #[test]
    fn test_valid_passport_true_1() {
        let mut passport = Passport::new();
        passport.insert(Field::EyeColor, String::from("gry"));
        passport.insert(Field::PassportID, String::from("860033327"));
        passport.insert(Field::ExpirationYear, String::from("2020"));
        passport.insert(Field::HairColor, String::from("#fffffd"));
        passport.insert(Field::BirthYear, String::from("1937"));
        passport.insert(Field::IssueYear, String::from("2017"));
        passport.insert(Field::CountryID, String::from("147"));
        passport.insert(Field::Height, String::from("183cm"));

        assert!(valid_passport(&passport, false));
    }

    #[test]
    fn test_valid_passport_true_2() {
        let mut passport = Passport::new();
        passport.insert(Field::HairColor, String::from("#ae17e1"));
        passport.insert(Field::IssueYear, String::from("2013"));
        passport.insert(Field::ExpirationYear, String::from("2024"));
        passport.insert(Field::EyeColor, String::from("brn"));
        passport.insert(Field::PassportID, String::from("760753108"));
        passport.insert(Field::BirthYear, String::from("1931"));
        passport.insert(Field::Height, String::from("179cm"));

        assert!(valid_passport(&passport, false));
    }

    #[test]
    fn test_valid_passport_with_data_validation_true() {
        let mut passport = Passport::new();
        passport.insert(Field::PassportID, String::from("087499704"));
        passport.insert(Field::Height, String::from("74in"));
        passport.insert(Field::EyeColor, String::from("grn"));
        passport.insert(Field::IssueYear, String::from("2012"));
        passport.insert(Field::ExpirationYear, String::from("2030"));
        passport.insert(Field::BirthYear, String::from("1980"));
        passport.insert(Field::HairColor, String::from("#623a2f"));

        assert!(valid_passport(&passport, true));
    }

    #[test]
    fn test_valid_passport_false() {
        let mut passport = Passport::new();
        passport.insert(Field::IssueYear, String::from("2013"));
        passport.insert(Field::EyeColor, String::from("amb"));
        passport.insert(Field::CountryID, String::from("350"));
        passport.insert(Field::ExpirationYear, String::from("2023"));
        passport.insert(Field::PassportID, String::from("028048884"));
        passport.insert(Field::HairColor, String::from("#cfa07d"));
        passport.insert(Field::BirthYear, String::from("1929"));

        assert!(!valid_passport(&passport, false));
    }

    #[test]
    fn test_valid_passport_with_data_validation_false() {
        let mut passport = Passport::new();
        passport.insert(Field::Height, String::from("59cm"));
        passport.insert(Field::EyeColor, String::from("zzz"));
        passport.insert(Field::ExpirationYear, String::from("2038"));
        passport.insert(Field::HairColor, String::from("74454a"));
        passport.insert(Field::IssueYear, String::from("2023"));
        passport.insert(Field::PassportID, String::from("3556412378"));
        passport.insert(Field::BirthYear, String::from("2007"));

        assert!(!valid_passport(&passport, true));
    }

    #[test]
    fn test_valid_birth_year_true() {
        assert!(valid_birth_year(&String::from("2002")));
    }

    #[test]
    fn test_valid_year_false() {
        assert!(!valid_birth_year(&String::from("2003")));
    }

    #[test]
    fn test_valid_height_inches_true() {
        assert!(valid_height(&String::from("60in")));
    }

    #[test]
    fn test_valid_height_centimeters_true() {
        assert!(valid_height(&String::from("190cm")));
    }

    #[test]
    fn test_valid_height_inches_false() {
        assert!(!valid_height(&String::from("190in")));
    }

    #[test]
    fn test_valid_height_false() {
        assert!(!valid_height(&String::from("190")));
    }

    #[test]
    fn test_valid_hair_color_true() {
        assert!(valid_hair_color(&String::from("#123abc")));
    }

    #[test]
    fn test_valid_hair_color_outside_of_hex_false() {
        assert!(!valid_hair_color(&String::from("#123abz")));
    }

    #[test]
    fn test_valid_hair_color_no_hash_false() {
        assert!(!valid_hair_color(&String::from("123abc")));
    }

    #[test]
    fn test_valid_eye_color_true() {
        assert!(valid_eye_color(&String::from("brn")));
    }

    #[test]
    fn test_valid_eye_color_false() {
        assert!(!valid_eye_color(&String::from("wat")));
    }

    #[test]
    fn test_valid_passport_id_true() {
        assert!(valid_passport_id(&String::from("000000001")));
    }

    #[test]
    fn test_valid_passport_id_false() {
        assert!(!valid_passport_id(&String::from("0123456789")));
    }
}
