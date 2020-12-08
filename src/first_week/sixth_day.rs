use std::collections::HashSet;
use std::iter::FromIterator;

fn collect_answers(input: Vec<String>) -> Vec<HashSet<char>> {
    let mut answer_set: HashSet<char> = HashSet::new();
    let mut groups = Vec::new();
    for inp in input {
        if inp.is_empty() {
            groups.push(answer_set);
            answer_set = HashSet::new();
            continue;
        }
        answer_set.extend(inp.chars());
    }
    groups.push(answer_set);
    groups
}

fn collect_correct_answers(input: Vec<String>) -> Vec<HashSet<char>> {
    let mut answer_set: HashSet<char> = HashSet::new();
    let mut groups = Vec::new();
    let mut first = true;
    for inp in input {
        if inp.is_empty() {
            groups.push(answer_set);
            answer_set = HashSet::new();
            first = true;
            continue;
        }
        if answer_set.is_empty() && first {
            answer_set.extend(inp.chars());
            first = false;
        } else {
            answer_set = answer_set
                .intersection(&HashSet::from_iter(inp.chars()))
                .cloned()
                .collect();
            first = false;
        }
    }
    groups.push(answer_set);
    groups
}

pub(super) fn sum_answers(input: Vec<String>, correct: bool) -> usize {
    if correct {
        collect_correct_answers(input)
            .into_iter()
            .map(|s| s.len())
            .sum()
    } else {
        collect_answers(input).into_iter().map(|s| s.len()).sum()
    }
}

#[test]
fn test_collect_answers() {
    let input = vec![
        "abc".to_string(),
        "".to_string(),
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "".to_string(),
        "ab".to_string(),
        "ac".to_string(),
        "".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "".to_string(),
        "b".to_string(),
    ];
    let answers = collect_answers(input);
    assert_eq!(5, answers.len());
    assert_eq!(3, answers[0].len());
    assert_eq!(3, answers[1].len());
    assert_eq!(3, answers[2].len());
    assert_eq!(1, answers[3].len());
    assert_eq!(1, answers[4].len());
}

#[test]
fn test_collect_correct_answers() {
    let input = vec![
        "abc".to_string(),
        "".to_string(),
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "".to_string(),
        "ab".to_string(),
        "ac".to_string(),
        "".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "".to_string(),
        "b".to_string(),
    ];
    let answers = collect_correct_answers(input);
    assert_eq!(5, answers.len());
    assert_eq!(3, answers[0].len());
    assert_eq!(0, answers[1].len());
    assert_eq!(1, answers[2].len());
    assert_eq!(1, answers[3].len());
    assert_eq!(1, answers[4].len());
}
#[test]
fn test_sum_answers() {
    let input = vec![
        "abc".to_string(),
        "".to_string(),
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "".to_string(),
        "ab".to_string(),
        "ac".to_string(),
        "".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "".to_string(),
        "b".to_string(),
    ];
    assert_eq!(11, sum_answers(input, false))
}

#[test]
fn test_sum_correct_answers() {
    let input = vec![
        "abc".to_string(),
        "".to_string(),
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "".to_string(),
        "ab".to_string(),
        "ac".to_string(),
        "".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "".to_string(),
        "b".to_string(),
    ];
    assert_eq!(6, sum_answers(input, true))
}
