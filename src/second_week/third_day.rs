use std::collections::HashMap;

fn find_differences(input: Vec<String>) -> (u64, u64, u64) {
    let mut values: Vec<u64> = input.iter().map(|s| s.parse().unwrap_or(0)).collect();
    values.sort_unstable();
    values
        .iter()
        .enumerate()
        .fold((0, 0, 1), |(mut acc1, mut acc2, mut acc3), (i, v)| {
            let diff;
            if i == 0 {
                diff = *v;
            } else {
                diff = v - values[i - 1];
            }
            match diff {
                1 => acc1 += 1,
                2 => acc2 += 1,
                3 => acc3 += 1,
                _ => panic!("not possible to connect adapters"),
            }
            (acc1, acc2, acc3)
        })
}

fn recurse_posibilities<'a>(
    start: u64,
    adapters: &'a [u64],
    used: &mut HashMap<&'a [u64], u64>,
) -> u64 {
    if adapters.is_empty() {
        return 1;
    }
    if let Some(count) = used.get(adapters) {
        return *count;
    }
    let total = adapters
        .iter()
        .take_while(|a| **a <= start + 3)
        .enumerate()
        .map(|(i, a)| recurse_posibilities(*a, &adapters[i + 1..], used))
        .sum();
    used.insert(adapters, total);
    total
}

pub fn mult_differece(input: Vec<String>) -> u64 {
    let (acc1, _, acc3) = find_differences(input);
    acc1 * acc3
}

pub fn find_possibilities(input: Vec<String>) -> u64 {
    let mut adapters: Vec<u64> = input.into_iter().map(|s| s.parse().unwrap_or(0)).collect();
    adapters.sort_unstable();
    let mut used = HashMap::new();
    recurse_posibilities(0, &adapters, &mut used)
}

#[test]
fn test_find_diffrences() {
    let input = vec![
        "28".to_string(),
        "33".to_string(),
        "18".to_string(),
        "42".to_string(),
        "31".to_string(),
        "14".to_string(),
        "46".to_string(),
        "20".to_string(),
        "48".to_string(),
        "47".to_string(),
        "24".to_string(),
        "23".to_string(),
        "49".to_string(),
        "45".to_string(),
        "19".to_string(),
        "38".to_string(),
        "39".to_string(),
        "11".to_string(),
        "1".to_string(),
        "32".to_string(),
        "25".to_string(),
        "35".to_string(),
        "8".to_string(),
        "17".to_string(),
        "7".to_string(),
        "9".to_string(),
        "4".to_string(),
        "2".to_string(),
        "34".to_string(),
        "10".to_string(),
        "3".to_string(),
    ];
    let result = find_differences(input);
    assert_eq!(22, result.0);
    assert_eq!(0, result.1);
    assert_eq!(10, result.2);
}

#[test]
fn test_mult_diffrences() {
    let input = vec![
        "28".to_string(),
        "33".to_string(),
        "18".to_string(),
        "42".to_string(),
        "31".to_string(),
        "14".to_string(),
        "46".to_string(),
        "20".to_string(),
        "48".to_string(),
        "47".to_string(),
        "24".to_string(),
        "23".to_string(),
        "49".to_string(),
        "45".to_string(),
        "19".to_string(),
        "38".to_string(),
        "39".to_string(),
        "11".to_string(),
        "1".to_string(),
        "32".to_string(),
        "25".to_string(),
        "35".to_string(),
        "8".to_string(),
        "17".to_string(),
        "7".to_string(),
        "9".to_string(),
        "4".to_string(),
        "2".to_string(),
        "34".to_string(),
        "10".to_string(),
        "3".to_string(),
    ];
    assert_eq!(220, mult_differece(input))
}

#[test]
fn test_find_possibilities() {
    let input = vec![
        "28".to_string(),
        "33".to_string(),
        "18".to_string(),
        "42".to_string(),
        "31".to_string(),
        "14".to_string(),
        "46".to_string(),
        "20".to_string(),
        "48".to_string(),
        "47".to_string(),
        "24".to_string(),
        "23".to_string(),
        "49".to_string(),
        "45".to_string(),
        "19".to_string(),
        "38".to_string(),
        "39".to_string(),
        "11".to_string(),
        "1".to_string(),
        "32".to_string(),
        "25".to_string(),
        "35".to_string(),
        "8".to_string(),
        "17".to_string(),
        "7".to_string(),
        "9".to_string(),
        "4".to_string(),
        "2".to_string(),
        "34".to_string(),
        "10".to_string(),
        "3".to_string(),
    ];
    assert_eq!(19208, find_possibilities(input))
}
