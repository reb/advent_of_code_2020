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
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_04.txt");

pub fn run() {
    println!("Not implemented yet");
    unimplemented!();
}

type Passport = HashMap<Field, String>;

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
}
