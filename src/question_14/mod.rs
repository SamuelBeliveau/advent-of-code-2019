use regex::Regex;
use std::str::FromStr;
use crate::util::read_content;
use std::collections::HashMap;

pub fn solve_a() {
    let contents = read_content("src/question_14/input.txt");
    let mut recipe: HashMap<_, _> = contents.lines()
        .map(|line| parse_reaction(line))
        .map(|reaction| (reaction.output.name.clone(), reaction))
        .collect();

    recipe.insert("ORE".to_string(), Reaction { inputs: Vec::new(), output: Ingredient::new("ORE".to_string(), 1) });

    let mut store = HashMap::new();

    required_quantities(&recipe, &mut store, &"FUEL".to_string());

    println!("Total: {}", store.get(&"ORE".to_string()).unwrap().1);
}

fn required_quantities(recipe: &HashMap<String, Reaction>, store: &mut HashMap<String, (u64, u64)>, ingredient_name: &String) {
    if let Some(reaction) = recipe.get(ingredient_name) {
        for input in &reaction.inputs {
            println!("Need {} of {}", input.quantity, input.name);
            let stored = store.entry(input.name.clone()).or_insert((0, 0));

            if stored.0 >= input.quantity {
                stored.0 -= input.quantity;
                continue;
            }

            println!("Not enough in store to fully cover ({})...", stored.0);
            let new_quantity = input.quantity - stored.0;
            stored.0 = 0;
            println!("Getting what we can from the store, new qty is {}...", new_quantity);

            let ingredient_reaction = recipe.get(&input.name).unwrap();
            let times = (new_quantity as f64 / ingredient_reaction.output.quantity as f64).ceil() as u64;
            stored.0 += (times * ingredient_reaction.output.quantity) - new_quantity;
            stored.1 += times * ingredient_reaction.output.quantity;
            for _ in 0..times {
                required_quantities(recipe, store, &input.name);
            }
        }

        return;
    }

    println!("No reaction for {}", ingredient_name);
    println!("Store: {:#?}", store);
}

fn parse_reaction(line: &str) -> Reaction {
    let re = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
    let ingredients: Vec<_> = re.captures_iter(line)
        .map(|capture| Ingredient::new(capture[2].to_string(), u64::from_str(&capture[1]).unwrap()))
        .collect();

    Reaction {
        inputs: ingredients[..ingredients.len() - 1].to_vec(),
        output: ingredients[ingredients.len() - 1].clone(),
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Ingredient {
    quantity: u64,
    name: String,
}

impl Ingredient {
    pub fn new(name: String, quantity: u64) -> Ingredient {
        Ingredient {
            name,
            quantity,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reaction {
    inputs: Vec<Ingredient>,
    output: Ingredient,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reaction() {
        let string = "16 PJXQB, 20 SDNCK, 3 HQFB, 7 QXGZ, 2 KNVN, 9 KZCVX => 8 XGTL";
        assert_eq!(parse_reaction(string), Reaction {
            inputs: vec![
                Ingredient::new("PJXQB".to_string(), 16),
                Ingredient::new("SDNCK".to_string(), 20),
                Ingredient::new("HQFB".to_string(), 3),
                Ingredient::new("QXGZ".to_string(), 7),
                Ingredient::new("KNVN".to_string(), 2),
                Ingredient::new("KZCVX".to_string(), 9)
            ],
            output: Ingredient::new("XGTL".to_string(), 8),
        });
    }
}