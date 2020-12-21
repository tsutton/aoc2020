use std::collections::{HashMap, HashSet};

struct Line {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let paren_index = line.find('(').unwrap();
            let ingredients = line[..paren_index]
                .trim()
                .split(' ')
                .map(|x| x.to_owned())
                .collect();
            // "(contains ".len() = 10
            // -1 from the end for ')'
            let allergens = line[paren_index + 10..line.len() - 1]
                .trim()
                .split(", ")
                .map(|x| x.to_owned())
                .collect();
            Line {
                ingredients,
                allergens,
            }
        })
        .collect()
}

#[allow(dead_code)]
const EXAMPLE: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

/// In the initial pass, we will make a tentative map of allergen: Vec<possible ingredients>
/// such that we can be 1005 sure that the correct ingredients is somewhere in the value for that key
/// We'll do this by taking, for each allergen, the intersection of all the ingredient lists such that
/// the recipe contains that allergen
fn initial_pass(lines: &[Line]) -> HashMap<String, HashSet<String>> {
    let mut possible_ingredients_per_allergen: HashMap<String, HashSet<String>> = HashMap::new();
    for line in lines {
        for allergen in line.allergens.iter() {
            let line_ingreds = line.ingredients.iter().cloned().collect();
            if possible_ingredients_per_allergen.contains_key(allergen) {
                // remove any possibilities that aren't on this line
                let intersection = possible_ingredients_per_allergen
                    .get(allergen)
                    .unwrap()
                    .intersection(&line_ingreds)
                    .map(|x| x.to_string())
                    .collect();
                possible_ingredients_per_allergen.insert(allergen.to_string(), intersection);
            } else {
                // initialize with all ingreds on this line
                possible_ingredients_per_allergen.insert(allergen.to_string(), line_ingreds);
            }
        }
    }
    possible_ingredients_per_allergen
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    // let input = EXAMPLE;
    let lines = parse(input);
    let possible_ingredients_per_allergen: HashMap<String, HashSet<String>> = initial_pass(&lines);
    // at this point, as per this print statement, there's still a bunch of possibilities
    // my guess is part 2 will give us more info to narrow it down
    println!("{:?}", possible_ingredients_per_allergen);
    // but we can maybe solve part 1 by iterating over all ingreds and finding the ones
    // that don't appear in any value in the hashmap
    let mut all_possibilites = HashSet::new();
    for p in possible_ingredients_per_allergen.values() {
        all_possibilites.extend(p);
    }
    println!("{:?}", all_possibilites);
    let mut answer = 0;
    for line in &lines {
        for ingred in &line.ingredients {
            if !all_possibilites.contains(ingred) {
                answer += 1;
            }
        }
    }
    answer
}

/// based on the print statement from part 1, we can see that this day is like that previous
/// day where we can solve the logic/constraint problem by simply finding the allergen that
/// has only one possibility, assigning it that one, then removing that ingredient from the
/// other allergens
#[aoc(day21, part2)]
pub fn part2(input: &str) -> String {
    // let input = EXAMPLE;
    let lines = parse(input);
    let mut possible_ingredients_per_allergen: HashMap<String, HashSet<String>> =
        initial_pass(&lines);
    let mut ingredients = HashMap::new();
    while !possible_ingredients_per_allergen.is_empty() {
        let (next_allergen, ingred) = possible_ingredients_per_allergen
            .iter()
            .find_map(|(allergen, set)| {
                if set.len() == 1 {
                    Some((
                        allergen.to_string(),
                        set.iter().collect::<Vec<_>>()[0].clone(),
                    ))
                } else {
                    None
                }
            })
            .unwrap();
        println!("assigning {} to {}", &ingred, &next_allergen);
        possible_ingredients_per_allergen.remove(&next_allergen);
        for value in possible_ingredients_per_allergen.values_mut() {
            value.remove(&ingred);
        }
        ingredients.insert(next_allergen, ingred);
    }
    let mut ingredients: Vec<_> = ingredients.into_iter().collect();
    ingredients.sort_unstable_by_key(|pair| pair.0.clone());
    println!("{:?}", ingredients);
    ingredients
        .into_iter()
        .map(|pair| pair.1)
        .collect::<Vec<_>>()
        .join(",")
}
