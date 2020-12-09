use std::collections::{HashMap, HashSet};

fn create_preamble(input: Vec<String>, pmb_num: usize) -> (HashMap<i32, HashSet<i32>>, Vec<i32>) {
    let input: Vec<i32> = input.into_iter().map(|s| s.parse().unwrap_or(0)).collect();
    (
        input
            .iter()
            .enumerate()
            .filter(|(i, _)| i >= &pmb_num)
            .map(|(i, n)| (*n, input[i - pmb_num..i].iter().cloned().collect()))
            .collect(),
        input,
    )
}

fn find_in_preamble(num: &i32, preamble: &HashSet<i32>) -> bool {
    for n in preamble {
        if preamble.contains(&(num - n)) {
            return true;
        }
    }
    false
}

fn find_range(num: i32, list: Vec<i32>) -> Option<i32> {
    for (i, _) in list.iter().enumerate() {
        let mut acc = 0;
        for (oi, n) in list[i..].iter().enumerate() {
            acc += n;
            if acc == num {
                let mut range = list[i..=oi + i].to_vec();
                range.sort_unstable();
                return Some(range[0] + range[range.len() - 1]);
            }
            if acc > num {
                break;
            }
        }
    }
    None
}

pub fn find_number(input: Vec<String>, pmb_num: usize) -> Option<i32> {
    let (preamble, _) = create_preamble(input, pmb_num);
    preamble
        .into_iter()
        .find(|(n, p)| !find_in_preamble(n, &p))
        .map(|(n, _)| n)
}

pub fn find_number_in_range(input: Vec<String>, pmb_num: usize) -> Option<i32> {
    let (preamble, list) = create_preamble(input, pmb_num);
    if let Some(n) = preamble
        .into_iter()
        .find(|(n, p)| !find_in_preamble(n, &p))
        .map(|(n, _)| n)
    {
        return find_range(n, list);
    }
    None
}

#[test]
fn test_create_preamble() {
    let input = vec![
        "35".to_string(),
        "20".to_string(),
        "15".to_string(),
        "25".to_string(),
        "47".to_string(),
        "40".to_string(),
        "62".to_string(),
        "55".to_string(),
        "65".to_string(),
        "95".to_string(),
        "102".to_string(),
        "117".to_string(),
        "150".to_string(),
        "182".to_string(),
        "127".to_string(),
        "219".to_string(),
        "299".to_string(),
        "277".to_string(),
        "309".to_string(),
        "576".to_string(),
    ];
    let (preamble, _) = create_preamble(input, 5);
    assert_eq!(15, preamble.len());
    assert_eq!(Some(&20), preamble[&40].get(&20));
    assert_eq!(None, preamble.get(&20));
    assert_eq!(Some(&127), preamble[&309].get(&127));
    assert_eq!(None, preamble[&127].get(&47));
}

#[test]
fn test_find_in_preamble() {
    let input = vec![
        "35".to_string(),
        "20".to_string(),
        "15".to_string(),
        "25".to_string(),
        "47".to_string(),
        "40".to_string(),
        "62".to_string(),
        "55".to_string(),
        "65".to_string(),
        "95".to_string(),
        "102".to_string(),
        "117".to_string(),
        "150".to_string(),
        "182".to_string(),
        "127".to_string(),
        "219".to_string(),
        "299".to_string(),
        "277".to_string(),
        "309".to_string(),
        "576".to_string(),
    ];
    let (preamble, _) = create_preamble(input, 5);
    assert!(find_in_preamble(&62, &preamble[&62]));
    assert!(find_in_preamble(&150, &preamble[&150]));
    assert!(find_in_preamble(&576, &preamble[&576]));
    assert!(!find_in_preamble(&127, &preamble[&127]))
}

#[test]
fn test_find_number() {
    let input = vec![
        "35".to_string(),
        "20".to_string(),
        "15".to_string(),
        "25".to_string(),
        "47".to_string(),
        "40".to_string(),
        "62".to_string(),
        "55".to_string(),
        "65".to_string(),
        "95".to_string(),
        "102".to_string(),
        "117".to_string(),
        "150".to_string(),
        "182".to_string(),
        "127".to_string(),
        "219".to_string(),
        "299".to_string(),
        "277".to_string(),
        "309".to_string(),
        "576".to_string(),
    ];
    assert_eq!(Some(127), find_number(input, 5))
}

#[test]
fn test_find_number_in_range() {
    let input = vec![
        "35".to_string(),
        "20".to_string(),
        "15".to_string(),
        "25".to_string(),
        "47".to_string(),
        "40".to_string(),
        "62".to_string(),
        "55".to_string(),
        "65".to_string(),
        "95".to_string(),
        "102".to_string(),
        "117".to_string(),
        "150".to_string(),
        "182".to_string(),
        "127".to_string(),
        "219".to_string(),
        "299".to_string(),
        "277".to_string(),
        "309".to_string(),
        "576".to_string(),
    ];
    assert_eq!(Some(62), find_number_in_range(input, 5))
}

#[test]
fn test_find_range() {
    let list = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(Some(62), find_range(127, list))
}
