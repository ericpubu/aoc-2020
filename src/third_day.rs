pub fn tree_map(input: Vec<String>, line_step: usize, position_step: usize) -> usize {
    let map: Vec<Vec<char>> = input
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let mut index = 0;
    let line_size = map.get(0).unwrap().len();

    (line_step..map.len())
        .step_by(line_step)
        .fold(0, |acc, il| {
            index += position_step;
            if index > line_size - 1 {
                index -= line_size;
            }
            if let Some(line) = map.get(il) {
                if let Some(position) = line.get(index) {
                    if *position == '#' {
                        return acc + 1;
                    }
                }
            }
            acc
        })
}

pub fn check_slots(input: Vec<String>, attempts: Vec<(usize, usize)>) -> usize {
    attempts
        .into_iter()
        .fold(1, |acc, pair| acc * tree_map(input.clone(), pair.0, pair.1))
}

//..##.........##.........##.........##.........##.........##.......  --->
//#..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//.#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//.#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//.#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//.#........#.#........X.#........#.#........#.#........#.#........#
//#.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//#...##....##...##....##...#X....##...##....##...##....##...##....#
//.#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
#[test]
fn test_tree_map() {
    let input = vec![
        "..##.......".to_string(),
        "#...#...#..".to_string(),
        ".#....#..#.".to_string(),
        "..#.#...#.#".to_string(),
        ".#...##..#.".to_string(),
        "..#.##.....".to_string(),
        ".#.#.#....#".to_string(),
        ".#........#".to_string(),
        "#.##...#...".to_string(),
        "#...##....#".to_string(),
        ".#..#...#.#".to_string(),
    ];
    assert_eq!(7, tree_map(input, 1, 3))
}

#[test]
fn test_check_slots_map() {
    let input = vec![
        "..##.......".to_string(),
        "#...#...#..".to_string(),
        ".#....#..#.".to_string(),
        "..#.#...#.#".to_string(),
        ".#...##..#.".to_string(),
        "..#.##.....".to_string(),
        ".#.#.#....#".to_string(),
        ".#........#".to_string(),
        "#.##...#...".to_string(),
        "#...##....#".to_string(),
        ".#..#...#.#".to_string(),
    ];
    let pairs: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    assert_eq!(336, check_slots(input, pairs))
}
