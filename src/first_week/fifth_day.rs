use std::iter::FromIterator;

fn find_seat_ids(input: Vec<String>) -> Vec<usize> {
    let rows: Vec<usize> = Vec::from_iter(0..128);
    let columns: Vec<usize> = Vec::from_iter(0..8);
    input
        .into_iter()
        // binary search
        .map(|s| {
            let mut row = rows.clone();
            let mut column = columns.clone();
            let row_mov = s.get(0..7).unwrap();
            let column_mov = s.get(7..).unwrap();
            row_mov.chars().into_iter().for_each(|m| {
                let (head, tail) = row.split_at(row.len() / 2);
                if m == 'F' {
                    row = head.to_vec();
                } else {
                    row = tail.to_vec();
                }
            });
            column_mov.chars().into_iter().for_each(|m| {
                let (head, tail) = column.split_at(column.len() / 2);
                if m == 'L' {
                    column = head.to_vec();
                } else {
                    column = tail.to_vec();
                }
            });
            (*row.get(0).unwrap(), *column.get(0).unwrap())
        })
        // Get ID
        .map(|(r, c)| (r * 8) + c)
        .collect()
}

pub(super) fn highest_seat_id(input: Vec<String>) -> usize {
    find_seat_ids(input).into_iter().max().unwrap_or(0)
}

pub(super) fn user_seat_id(input: Vec<String>) -> usize {
    let mut ids = find_seat_ids(input);
    ids.sort_unstable();
    let first = ids[0];
    ids.into_iter().fold(first, |acc, curr| {
        if curr - acc > 1 {
            return acc;
        }
        curr
    }) + 1
}

#[test]
fn test_find_seat_ids() {
    let input = vec![
        "FBFBBFFRLR".to_string(),
        "BFFFBBFRRR".to_string(),
        "FFFBBBFRRR".to_string(),
        "BBFFBBFRLL".to_string(),
    ];
    let ids = find_seat_ids(input);
    assert_eq!(4, ids.len());
    assert_eq!(357, ids[0]);
    assert_eq!(567, ids[1]);
    assert_eq!(119, ids[2]);
    assert_eq!(820, ids[3]);
}

#[test]
fn test_highest_id() {
    let input = vec![
        "FBFBBFFRLR".to_string(),
        "BFFFBBFRRR".to_string(),
        "FFFBBBFRRR".to_string(),
        "BBFFBBFRLL".to_string(),
    ];
    assert_eq!(820, highest_seat_id(input))
}
