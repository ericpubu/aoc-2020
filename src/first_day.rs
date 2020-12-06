use std::collections::HashSet;
use std::iter::FromIterator;

pub fn expenses(input: Vec<i32>) -> i32 {
    let objective = 2020;
    let set: HashSet<i32> = HashSet::from_iter(input.iter().cloned());
    for value in input {
        let key = objective - value;
        if let Some(result) = set.get(&key) {
            return result * value;
        }
    }
    0
}

pub fn expenses2(input: Vec<i32>) -> i32 {
    let objective = 2020;
    let set: HashSet<i32> = HashSet::from_iter(input.iter().cloned());
    for value in &input {
        let new_objective: i32 = objective - value;
        for v in &input {
            let key: i32 = new_objective - v;
            if let Some(result) = set.get(&key) {
                return result * value * v;
            }
        }
    }
    0
}

#[test]
fn test_expenses() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(514579, expenses(input))
}

#[test]
fn test_expenses2() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(241861950, expenses2(input))
}
