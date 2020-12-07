use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

lazy_static! {
    static ref VALID_HLC: Regex = Regex::new(r"^#[a-fA-F0-9]+$").expect("invalid regex");
    static ref VALID_ECL: Regex =
        Regex::new(r"\b(?:amb|blu|brn|gry|grn|hzl|oth)\b").expect("invalid regex");
}

#[derive(Debug, Deserialize, Serialize, Validate)]
struct Passport {
    #[validate(required, range(min = 1920, max = 2002))]
    byr: Option<i32>,
    #[validate(required, range(min = 2010, max = 2020))]
    iyr: Option<i32>,
    #[validate(required, range(min = 2020, max = 2030))]
    eyr: Option<i32>,
    #[validate(required, custom = "validate_height")]
    hgt: Option<Height>,
    #[validate(required, length(equal = 7), regex = "VALID_HLC")]
    hcl: Option<String>,
    #[validate(required, length(equal = 3), regex = "VALID_ECL")]
    ecl: Option<String>,
    #[validate(required, length(equal = 9))]
    pid: Option<String>,
    cid: Option<String>,
}

fn validate_height(height: &Height) -> Result<(), ValidationError> {
    if height.unit == "cm" && height.number >= 150 && height.number <= 193 {
        return Ok(());
    }
    if height.unit == "in" && height.number >= 59 && height.number <= 76 {
        return Ok(());
    }
    Err(ValidationError::new("invalid_height"))
}

#[derive(Debug, Deserialize, Serialize)]
struct Height {
    unit: String,
    number: i16,
}

impl Passport {
    fn new() -> Self {
        Passport {
            cid: None,
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        }
    }

    fn is_complete(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    //FIXME: Find a better way to serialize, probably with serde, this is too manual.
    fn insert_fields(&mut self, fields: Vec<(&str, &str)>) {
        for field in fields {
            if field.0 == "cid" {
                self.cid = Some(field.1.to_string());
            }
            if field.0 == "byr" {
                self.byr = field.1.parse().ok();
            }
            if field.0 == "iyr" {
                self.iyr = field.1.parse().ok();
            }
            if field.0 == "eyr" {
                self.eyr = field.1.parse().ok();
            }
            if field.0 == "hgt" {
                let v: String = field.1.chars().rev().collect();
                if let Some(unit) = v.get(0..2) {
                    let number: Option<i16> = v
                        .get(2..)
                        .map(|n| n.chars().rev().collect::<String>().parse().unwrap_or(0));
                    if number.is_none() {
                        continue;
                    }
                    let height = Height {
                        unit: unit.chars().rev().collect(),
                        number: number.unwrap(),
                    };
                    self.hgt = Some(height);
                }
            }
            if field.0 == "hcl" {
                self.hcl = Some(field.1.to_string());
            }
            if field.0 == "ecl" {
                self.ecl = Some(field.1.to_string());
            }
            if field.0 == "cid" {
                self.cid = Some(field.1.to_string());
            }
            if field.0 == "pid" {
                self.pid = Some(field.1.to_string());
            }
        }
    }
}

fn create_passports(input: Vec<String>) -> Vec<Passport> {
    let mut passport = Passport::new();
    let mut passports = Vec::new();
    for inp in input {
        if inp.is_empty() {
            passports.push(passport);
            passport = Passport::new();
            continue;
        }
        let fields: Vec<(&str, &str)> = inp
            .split(' ')
            .map(|s| {
                let mut field = s.split(':');
                (field.next().unwrap(), field.next().unwrap())
            })
            .collect();
        passport.insert_fields(fields);
    }
    passports.push(passport);
    passports
}

pub fn validate_passport(input: Vec<String>) -> u32 {
    create_passports(input).into_iter().fold(
        0,
        |acc, passport| {
            if passport.is_complete() {
                acc + 1
            } else {
                acc
            }
        },
    )
}

pub fn validate_passport_content(input: Vec<String>) -> u32 {
    create_passports(input)
        .into_iter()
        .fold(0, |acc, passport| {
            if passport.validate().is_ok() {
                acc + 1
            } else {
                acc
            }
        })
}
#[test]
fn test_create_passports() {
    let input = vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
        "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
        "".to_string(),
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
        "hcl:#cfa07d byr:1929".to_string(),
        "".to_string(),
        "hcl:#ae17e1 iyr:2013".to_string(),
        "eyr:2024".to_string(),
        "ecl:brn pid:760753108 byr:1931".to_string(),
        "hgt:179cm".to_string(),
        "".to_string(),
        "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
        "iyr:2011 ecl:brn hgt:59in".to_string(),
    ];
    let passports = create_passports(input);
    assert_eq!(4, passports.len());
    assert_eq!(true, passports[0].ecl.is_some());
    assert_eq!("gry".to_string(), passports[0].ecl.clone().unwrap());
    assert_eq!(true, passports[1].cid.is_some());
    assert_eq!("350".to_string(), passports[1].cid.clone().unwrap());
    assert_eq!(true, passports[2].iyr.is_some());
    assert_eq!(2013, passports[2].iyr.clone().unwrap());
    assert_eq!(true, passports[3].hcl.is_some());
    assert_eq!("#cfa07d".to_string(), passports[3].hcl.clone().unwrap());
}

#[test]
fn test_validate_passports() {
    let input = vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
        "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
        "".to_string(),
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
        "hcl:#cfa07d byr:1929".to_string(),
        "".to_string(),
        "hcl:#ae17e1 iyr:2013".to_string(),
        "eyr:2024".to_string(),
        "ecl:brn pid:760753108 byr:1931".to_string(),
        "hgt:179cm".to_string(),
        "".to_string(),
        "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
        "iyr:2011 ecl:brn hgt:59in".to_string(),
    ];

    assert_eq!(2, validate_passport(input))
}

#[test]
fn test_validate_passports_content() {
    let input = vec![
        "eyr:1972 cid:100".to_string(),
        "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
        "".to_string(),
        "iyr:2019".to_string(),
        "hcl:#602927 eyr:1967 hgt:170cm".to_string(),
        "ecl:grn pid:012533040 byr:1946".to_string(),
        "".to_string(),
        "hcl:dab227 iyr:2012".to_string(),
        "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
        "".to_string(),
        "hgt:59cm ecl:zzz".to_string(),
        "eyr:2038 hcl:74454a iyr:2023".to_string(),
        "pid:3556412378 byr:2007".to_string(),
        "".to_string(),
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
        "hcl:#623a2f".to_string(),
        "".to_string(),
        "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
        "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
        "".to_string(),
        "hcl:#888785".to_string(),
        "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
        "pid:545766238 ecl:hzl".to_string(),
        "eyr:2022".to_string(),
        "".to_string(),
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
    ];

    assert_eq!(4, validate_passport_content(input))
}
