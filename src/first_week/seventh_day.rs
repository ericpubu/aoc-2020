use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Bag {
    color: String,
    bags: Vec<(usize, String)>,
}

fn create_bags_map(input: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut bags_map = HashMap::new();
    let r = Regex::new(r"bags|bag|contain|\.|,").unwrap();
    for inp in input {
        let mut bags = r.split(inp.as_ref());
        if inp.contains("no") {
            bags_map.insert(bags.next().unwrap().trim().to_string(), Vec::new());
        } else {
            bags_map.insert(
                bags.next().unwrap().trim().to_string(),
                // TODO: Fix regex to avoid this filter
                bags.filter(|s| !s.is_empty() && !s.contains(',') && s != &" ")
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>(),
            );
        }
    }
    bags_map
}

fn create_bag(color: String, content: Vec<String>) -> Bag {
    if content.is_empty() {
        return Bag {
            color,
            bags: Vec::new(),
        };
    }
    let bags = content
        .into_iter()
        .map(|c| {
            let mut bs = c.split(' ');
            let num: usize = bs.next().map(|s| s.parse().unwrap_or(0)).unwrap();
            let contained_bag = bs.collect::<Vec<&str>>().join(" ");
            (num, contained_bag)
        })
        .collect();
    Bag { color, bags }
}

fn create_bags(input: Vec<String>) -> HashMap<String, Bag> {
    create_bags_map(input)
        .into_iter()
        .map(|(k, v)| (k.clone(), create_bag(k, v)))
        .collect()
}

fn search_colors(bag: &Bag, bag_color: &str, bags: &HashMap<String, Bag>) -> bool {
    if bag.color == bag_color {
        return true;
    }
    if bag.bags.is_empty() {
        return false;
    }
    for (_, b) in &bag.bags {
        if search_colors(bags.get(b).unwrap(), bag_color, bags) {
            return true;
        }
    }
    false
}

fn count_bags(bag: &Bag, bags: &HashMap<String, Bag>) -> usize {
    if bag.bags.is_empty() {
        return 0;
    }
    bag.bags
        .iter()
        .map(|(num, b)| (count_bags(bags.get(b).unwrap(), bags) * num) + num)
        .sum()
}

pub fn contained_bags_colors(input: Vec<String>, bag_color: &str) -> usize {
    let bags = create_bags(input);
    bags.iter().fold(0, |acc, (_, bag)| {
        if search_colors(bag, bag_color, &bags) {
            return acc + 1;
        }
        acc
    }) - 1 // Ignores the actual bag
}

pub fn contained_bags(input: Vec<String>, bag_color: &str) -> usize {
    let bags = create_bags(input);
    if let Some(bag) = bags.get(bag_color) {
        return bag
            .bags
            .iter()
            .map(|(num, b)| (count_bags(bags.get(b).unwrap(), &bags) * num) + num)
            .sum();
    }
    0
}

#[test]
fn test_create_bags_map() {
    let input = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
        "faded blue bags contain no other bags.".to_string(),
        "dotted black bags contain no other bags.".to_string(),
    ];
    let bm = create_bags_map(input);
    assert_eq!(9, bm.len());
    assert_eq!(0, bm.get("dotted black").unwrap().len());
    assert_eq!(2, bm.get("shiny gold").unwrap().len());
    assert_eq!(2, bm.get("light red").unwrap().len());
}

#[test]
fn test_create_bags() {
    let input = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
        "faded blue bags contain no other bags.".to_string(),
        "dotted black bags contain no other bags.".to_string(),
    ];
    let bags = create_bags(input);
    assert_eq!(9, bags.len());
    assert_eq!(0, bags.get("dotted black").unwrap().bags.len());
    assert_eq!(2, bags.get("shiny gold").unwrap().bags.len());
    assert_eq!(2, bags.get("light red").unwrap().bags.len());
    assert_eq!(
        "bright white".to_string(),
        bags.get("light red").unwrap().bags[0].1
    );
    assert_eq!(1, bags.get("light red").unwrap().bags[0].0);
}

#[test]
fn test_contained_bags_colors() {
    let input = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
        "faded blue bags contain no other bags.".to_string(),
        "dotted black bags contain no other bags.".to_string(),
    ];
    assert_eq!(4, contained_bags_colors(input, "shiny gold"))
}

#[test]
fn test_contained_bags() {
    let input = vec![
        "shiny gold bags contain 2 dark red bags.".to_string(),
        "dark red bags contain 2 dark orange bags.".to_string(),
        "dark orange bags contain 2 dark yellow bags.".to_string(),
        "dark yellow bags contain 2 dark green bags.".to_string(),
        "dark green bags contain 2 dark blue bags.".to_string(),
        "dark blue bags contain 2 dark violet bags.".to_string(),
        "dark violet bags contain no other bags.".to_string(),
    ];
    assert_eq!(126, contained_bags(input, "shiny gold"))
}
