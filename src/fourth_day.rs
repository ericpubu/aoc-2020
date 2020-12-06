#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
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

    fn is_valid(&self, accept_np: bool) -> bool {
        let mut np = true;
        if !accept_np {
            np = self.cid.is_some();
        }
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
            && np
    }

    //TODO: Find a better way to serialize, probably with serde, this is to manual.
    fn insert_fields(&mut self, fields: Vec<(&str, &str)>) {
        for field in fields {
            if field.0 == "cid" {
                self.cid = Some(field.1.to_string());
            }
            if field.0 == "byr" {
                self.byr = Some(field.1.to_string());
            }
            if field.0 == "iyr" {
                self.iyr = Some(field.1.to_string());
            }
            if field.0 == "eyr" {
                self.eyr = Some(field.1.to_string());
            }
            if field.0 == "hgt" {
                self.hgt = Some(field.1.to_string());
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
                //println!("{}", s);
                let mut field = s.split(':');
                //println!("{:?}", field);
                (field.next().unwrap(), field.next().unwrap())
            })
            .collect();
        passport.insert_fields(fields);
    }
    passports.push(passport);
    passports
}

pub fn validate_passport(input: Vec<String>) -> u32 {
    create_passports(input)
        .into_iter()
        .fold(0, |acc, passport| {
            if passport.is_valid(true) {
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
    assert_eq!("2013".to_string(), passports[2].iyr.clone().unwrap());
    assert_eq!(true, passports[3].hcl.is_some());
    assert_eq!("#cfa07d".to_string(), passports[3].hcl.clone().unwrap());
}

#[test]
fn test_validate_password() {
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
