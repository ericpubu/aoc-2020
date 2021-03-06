use regex::Regex;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Policy {
    first_number: usize,
    second_number: usize,
    character: char,
    password: String,
}

fn create_policies(list: Vec<String>) -> Vec<Policy> {
    let r = Regex::new(r"[\s.?!,-/:]+").unwrap();
    list.into_iter()
        .map(|s| {
            let parts: Vec<&str> = r.split(s.as_ref()).collect();
            Policy {
                first_number: parts.get(0).map(|s| s.parse::<usize>().unwrap()).unwrap(),
                second_number: parts.get(1).map(|s| s.parse::<usize>().unwrap()).unwrap(),
                character: parts.get(2).map(|s| s.chars().next().unwrap()).unwrap(),
                password: parts.get(3).map(|s| s.to_string()).unwrap(),
            }
        })
        .collect()
}

pub(super) fn incorrect_passwords(list: Vec<String>) -> u32 {
    create_policies(list).into_iter().fold(0, |acc, policy| {
        let times = policy.password.matches(policy.character).count();
        if times >= policy.first_number && times <= policy.second_number {
            return acc + 1;
        }
        acc
    })
}

pub(super) fn correct_passwords(list: Vec<String>) -> u32 {
    create_policies(list).into_iter().fold(0, |acc, policy| {
        let first_ch = policy
            .password
            .chars()
            .nth(policy.first_number - 1)
            .map(|c| c == policy.character)
            .unwrap_or(false);
        let second_ch = policy
            .password
            .chars()
            .nth(policy.second_number - 1)
            .map(|c| c == policy.character)
            .unwrap_or(false);
        if first_ch ^ second_ch {
            return acc + 1;
        }
        acc
    })
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
