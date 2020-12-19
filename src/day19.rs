use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Rule {
    Literal(String),
    Option(Box<Rule>, Box<Rule>),
    Concat(Vec<usize>),
}

impl Rule {
    fn parse(input: &str) -> (Rule, Vec<usize>) {
        let (deps, rule) = if let Some(left_quote) = input.find('"') {
            let right_quote = input[left_quote + 1..].find('"').unwrap() + left_quote + 1;
            (
                vec![],
                Rule::Literal(input[left_quote + 1..right_quote].to_owned()),
            )
        } else if let Some(pipe) = input.find('|') {
            let (left_rules, mut left_deps) = Rule::parse(&input[..pipe]);
            let (right_rules, right_deps) = Rule::parse(&input[pipe + 1..]);
            for v in right_deps {
                if !left_deps.contains(&v) {
                    left_deps.push(v);
                }
            }
            (
                left_deps,
                Rule::Option(Box::from(left_rules), Box::from(right_rules)),
            )
        } else {
            let parts: Vec<_> = input
                .trim()
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect();
            (parts.clone(), Rule::Concat(parts))
        };
        (rule, deps)
    }
}

pub fn parse_line(input: &str) -> (usize, Rule, Vec<usize>) {
    let colon_idx = input.find(':').unwrap();
    let rule_num: usize = input[..colon_idx].parse().unwrap();
    let rest = &input[colon_idx + 2..];
    let parsed = Rule::parse(rest);
    (rule_num, parsed.0, parsed.1)
}

pub struct Input {
    rules: HashMap<usize, Rule>,
    deps: HashMap<usize, Vec<usize>>,
}

pub fn parse(input: &str) -> Input {
    let mut ret = Input {
        rules: HashMap::new(),
        deps: HashMap::new(),
    };
    for line in input.lines() {
        let (rule_num, rule, deps) = parse_line(line);
        ret.rules.insert(rule_num, rule);
        let current_deps = ret.deps.entry(rule_num).or_insert(vec![]);
        for d in deps {
            if !current_deps.contains(&d) {
                current_deps.push(d);
            }
        }
    }
    ret
}

#[aoc(day19, part1)]
pub fn part1(i: &str) -> i32 {
    let divider = i.find("\n\n").unwrap();
    let input = parse(&i[..divider]);
    let mut cache = HashMap::new();
    part1::set_matches(0, &input.rules, &mut cache);
    let zero_matches = cache.get(&0).unwrap();

    let mut count = 0;
    let body = &i[divider + 2..];
    for line in body.lines() {
        if zero_matches.contains(line) {
            count += 1;
        }
    }
    count
}

/// This is the code as I wrote it for part 2, before working on part 2
/// In part 1, there are no cycles - the rules form a DAG, and the set of matches for each
/// rule is finite.
/// Our strategy is, then, to build up a map of (rule number) => (set of all matches)
/// We can do this recursively (using memoization) without fear of cycles.
/// Structurally, the two functions are `set_matches`, which takes a rule number, a collection
/// of rules, and cache, computes the set of all matches for that rule number, and stores it in the
/// cache - possible recursively calling set_matches.
/// The rules-engine is implemented by get_matches. This function takes a Rule, not a rule number,
/// because we have "anonymous" rules that we may want to match, e.g. in the rule `2: 4 4 | 5 5`,
/// we need to compute matches for the subrules `4 4` and `5 5`, but those don't have rule-numbers
mod part1 {
    use super::Rule;
    use std::collections::HashMap;
    use std::collections::HashSet;
    pub fn set_matches(
        rule_num: usize,
        all_rules: &HashMap<usize, Rule>,
        cache: &mut HashMap<usize, HashSet<String>>,
    ) {
        if cache.contains_key(&rule_num) {
            return;
        }
        let rule = all_rules.get(&rule_num).unwrap();
        let matches = get_matches(rule, all_rules, cache);
        cache.insert(rule_num, matches);
    }

    fn get_matches(
        rule: &Rule,
        all_rules: &HashMap<usize, Rule>,
        cache: &mut HashMap<usize, HashSet<String>>,
    ) -> HashSet<String> {
        match rule {
            Rule::Literal(s) => {
                let mut set = HashSet::new();
                set.insert(s.to_owned());
                set
            }
            Rule::Concat(rules) => {
                for r in rules.iter() {
                    set_matches(*r, all_rules, cache);
                }
                let mut matches = cache.get(&rules[0]).unwrap().clone();
                for next in rules[1..].iter() {
                    let mut new_matches = HashSet::new();
                    let suffixes = cache.get(next).unwrap();
                    for prefix in matches.iter() {
                        for suffix in suffixes {
                            let mut new = prefix.to_string();
                            new.push_str(suffix);
                            new_matches.insert(new);
                        }
                    }
                    matches = new_matches;
                }
                matches
            }
            Rule::Option(left, right) => {
                let mut left_matches = get_matches(left, all_rules, cache);
                for m in get_matches(right, all_rules, cache) {
                    left_matches.insert(m);
                }
                left_matches
            }
        }
    }
}

/// We're really going to take to heart the problem's advice to focus on the specific case
/// and not the general problem.
/// Looking closely at the input, we find that 8 depends on 42, 11 depends on 42 and 31, and
/// the entire rule-graph rooted in 42 and the one rooted in 31 are still DAGs: no cycles.
/// Furthermore, 0 depends on 8 and 11, but NOTHING else depends on those two.
/// Thus our plan will be to re-use the code from part 1 to find complete matching sets for 42 and 31
/// Then hardcode a simple strategy for each of 8 and 11 to loop over an input, divide it into chunks,
/// and match the chunks separately against the base rules.
/// This would completely break down if we had a cycle of rules where A depends on B and B depends on A
/// (or any longer length).
/// It could be extended to be slightly, teeny tiny bit, more general than this.
mod part2 {
    use std::collections::HashSet;

    pub fn matches_8(test: &str, matches_for_42: &HashSet<String>) -> bool {
        if matches_for_42.contains(test) {
            return true;
        }
        if test.is_empty() {
            return false;
        }
        for i in 1..test.len() - 1 {
            if matches_for_42.contains(&test[..i]) && matches_8(&test[i..], matches_for_42) {
                return true;
            }
        }
        false
    }

    pub fn matches_11(
        test: &str,
        matches_for_42: &HashSet<String>,
        matches_for_31: &HashSet<String>,
    ) -> bool {
        for i in 0..test.len() - 1 {
            for j in i..test.len() {
                let left = &test[..i];
                let middle = &test[i..j];
                let right = &test[j..];
                if matches_for_42.contains(left)
                    && matches_for_31.contains(right)
                    && (middle == "" || matches_11(middle, matches_for_42, matches_for_31))
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn matches_0(
        test: &str,
        matches_for_42: &HashSet<String>,
        matches_for_31: &HashSet<String>,
    ) -> bool {
        for i in 0..test.len() {
            if matches_8(&test[..i], matches_for_42)
                && matches_11(&test[i..], matches_for_42, matches_for_31)
            {
                return true;
            }
        }
        false
    }
}

#[aoc(day19, part2)]
pub fn part2(i: &str) -> i32 {
    let divider = i.find("\n\n").unwrap();
    let input = parse(&i[..divider]);
    let mut cache = HashMap::new();

    part1::set_matches(42, &input.rules, &mut cache);
    part1::set_matches(31, &input.rules, &mut cache);

    let mut count = 0;
    let body = &i[divider + 2..];
    for line in body.lines() {
        if part2::matches_0(line, cache.get(&42).unwrap(), cache.get(&31).unwrap()) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_parse() {
        let lines = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#;
        let lines: Vec<_> = lines.lines().collect();
        assert_eq!(
            parse_line(lines[0]),
            (0, Rule::Concat(vec![4, 1, 5]), vec![4, 1, 5])
        );
        assert_eq!(
            parse_line(lines[1]),
            (
                1,
                Rule::Option(
                    Box::from(Rule::Concat(vec![2, 3])),
                    Box::from(Rule::Concat(vec![3, 2])),
                ),
                vec![2, 3]
            )
        );
        assert_eq!(
            parse_line(lines[4]),
            (4, Rule::Literal("a".to_owned()), vec![])
        );
    }

    #[test]
    fn test_solve() {
        let lines = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#;
        let input = parse(lines);

        let mut cache = HashMap::new();
        part1::set_matches(5, &input.rules, &mut cache);
        assert_eq!(
            cache.get(&5),
            Some(&vec!["b".to_string()].into_iter().collect())
        );
        part1::set_matches(3, &input.rules, &mut cache);
        assert_eq!(
            cache.get(&3),
            Some(
                &vec!["ab".to_string(), "ba".to_string()]
                    .into_iter()
                    .collect()
            )
        );

        let zero_matches = vec![
            "aaaabb".to_string(),
            "aaabab".to_string(),
            "abbabb".to_string(),
            "abbbab".to_string(),
            "aabaab".to_string(),
            "aabbbb".to_string(),
            "abaaab".to_string(),
            "ababbb".to_string(),
        ];
        part1::set_matches(0, &input.rules, &mut cache);
        assert_eq!(cache.get(&0), Some(&zero_matches.into_iter().collect()));
    }

    #[test]
    fn test_recursive() {
        use part2::*;
        let matches_for_42: HashSet<_> = vec!["ba".to_string(), "aab".to_string()]
            .into_iter()
            .collect();

        let matches_for_31: HashSet<_> = vec!["bbb".to_string(), "bab".to_string()]
            .into_iter()
            .collect();

        assert!(matches_8("ba", &matches_for_42));
        assert!(matches_8("baba", &matches_for_42));
        assert!(matches_8("bababa", &matches_for_42));
        assert!(matches_8("baaab", &matches_for_42));
        assert!(matches_8("baaabba", &matches_for_42));

        assert!(!matches_8("b", &matches_for_42));
        assert!(!matches_8("bab", &matches_for_42));
        assert!(!matches_8("babab", &matches_for_42));
        assert!(!matches_8("baaa", &matches_for_42));
        assert!(!matches_8("baaabb", &matches_for_42));

        assert!(matches_11("babbb", &matches_for_42, &matches_for_31));
        assert!(matches_11("baaabbabbbb", &matches_for_42, &matches_for_31));
        assert!(!matches_11("bbb", &matches_for_42, &matches_for_31));
        assert!(!matches_11("baba", &matches_for_42, &matches_for_31));
    }
}
