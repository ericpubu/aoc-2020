use regex::Regex;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Policy {
    first_number: usize,
    second_number: usize,
    character: char,
    password: String,
}

fn create_policies(list: Vec<String>) -> Vec<Policy> {
    let mut policies: Vec<Policy> = Vec::with_capacity(list.len());
    let r = Regex::new(r"[\s.?!,-/:]+").unwrap();
    for l in list {
        let parts: Vec<&str> = r.split(l.as_ref()).collect();
        let dl = *parts.get(0).unwrap();
        let ul = *parts.get(1).unwrap();
        let ch = *parts.get(2).unwrap();
        let password = *parts.get(3).unwrap();
        policies.push(Policy {
            first_number: dl.parse().unwrap(),
            second_number: ul.parse().unwrap(),
            character: ch.chars().next().unwrap(),
            password: password.to_string(),
        })
    }
    policies
}

pub fn incorrect_passwords(list: Vec<String>) -> u32 {
    let policies = create_policies(list);
    let mut counter: u32 = 0;
    for policy in policies {
        let times = policy.password.matches(policy.character).count();
        if times >= policy.first_number && times <= policy.second_number {
            counter += 1;
        }
    }
    counter
}

pub fn correct_passwords(list: Vec<String>) -> u32 {
    let policies = create_policies(list);
    let mut correct: u32 = 0;
    for policy in policies {
        let mut occ: u32 = 0;
        if let Some(ch) = policy.password.chars().nth(policy.first_number - 1) {
            if ch == policy.character {
                occ += 1;
            }
        }
        if let Some(ch) = policy.password.chars().nth(policy.second_number - 1) {
            if ch == policy.character {
                occ += 1;
            }
        }
        if occ == 1 {
            correct += 1;
        }
    }
    correct
}

#[test]
fn test_create_policies() {
    let input = vec![
        "5-11 t: glhbttzvzttkdx".to_string(),
        "2-4 f: cfkmf".to_string(),
        "9-12 m: mmmmmmmmmmmmm".to_string(),
    ];
    let expected = vec![
        Policy {
            first_number: 5,
            second_number: 11,
            character: 't',
            password: "glhbttzvzttkdx".to_string(),
        },
        Policy {
            first_number: 2,
            second_number: 4,
            character: 'f',
            password: "cfkmf".to_string(),
        },
        Policy {
            first_number: 9,
            second_number: 12,
            character: 'm',
            password: "mmmmmmmmmmmmm".to_string(),
        },
    ];
    assert_eq!(expected, create_policies(input))
}

#[test]
fn test_incorrect_passwords() {
    let input = vec![
        "5-11 t: glhbttzvzttkdx".to_string(),
        "2-4 f: cfkmf".to_string(),
        "9-12 m: mmmmmmmmmmmmm".to_string(),
    ];
    assert_eq!(1, incorrect_passwords(input))
}

#[test]
fn test_correct_passwords() {
    let input = vec![
        "1-3 a: abcde".to_string(),
        "1-3 a: cbade".to_string(),
        "1-3 b: cdefg".to_string(),
        "2-9 c: ccccccccc".to_string(),
    ];
    assert_eq!(2, correct_passwords(input))
}
