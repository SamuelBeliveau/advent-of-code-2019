use crate::util::read_content;
use std::collections::HashMap;
use std::cmp::Ordering;

pub fn solve_a() {
    let contents = read_content("src/question_8/input.txt");
    let mut layers = extract_layers(&contents, 25, 6);

    let min_layer = layers.iter()
        .map(|layer| (layer, layer.color_counts.get(&0).unwrap_or_else(|| &0)))
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or_else(|| Ordering::Equal))
        .map(|(l, _)| l)
        .unwrap();

    let result = *min_layer.color_counts.get(&1).unwrap_or_else(|| &0) * *min_layer.color_counts.get(&2).unwrap_or_else(|| &0);
    println!("Result: {}", result);
}

pub fn solve_b() {
    let contents = read_content("src/question_8/input.txt");
    let mut layers = extract_layers(&contents, 25, 6);
    let flattened = flatten_layers(&layers);
    for i in 0..6 {
        for j in i * 25..(i + 1) * 25 {
            print!("{}", if flattened[j] == 1 { "#" } else { " " });
        }
        println!();
    }
}

fn flatten_layers(layers: &Vec<Layer>) -> Vec<u32> {
    let pixels_count = layers.get(0).map(|l| l.pixels.len()).unwrap_or_else(|| 0);
    let mut flattened = Vec::new();

    for i in 0..pixels_count {
        for j in 0..layers.len() {
            let pixel = layers[j].pixels[i];
            if pixel != 2 {
                flattened.push(pixel);
                break;
            }
        }
        if flattened.len() - 1 < i {
            flattened.push(2);
        }
    }

    flattened
}

fn extract_layers(contents: &String, width: usize, height: usize) -> Vec<Layer> {
    let mut layers = Vec::new();
    let mut current_layer = Layer::new();
    for (i, char) in contents.chars().enumerate() {
        if i > 0 && i % (width * height) == 0 {
            layers.push(current_layer);
            current_layer = Layer::new()
        }

        let pixel = char.to_digit(10).unwrap();

        current_layer.pixels.push(pixel);
        current_layer.color_counts.entry(pixel).and_modify(|v| *v += 1).or_insert(1);
    }
    layers.push(current_layer);
    layers
}

struct Layer {
    pixels: Vec<u32>,
    color_counts: HashMap<u32, u32>,
}

impl Layer {
    pub fn new() -> Layer {
        Layer {
            pixels: Vec::new(),
            color_counts: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_layers() {
        let content = "123455789012".to_string();
        let layers = extract_layers(&content, 3, 2);
        assert_eq!(layers.len(), 2);
        assert_eq!(layers[0].pixels, vec![1, 2, 3, 4, 5, 5]);
        assert_eq!(layers[0].color_counts.len(), 5);
        assert_eq!(layers[0].color_counts.get(&1).unwrap(), &1);
        assert_eq!(layers[0].color_counts.get(&5).unwrap(), &2);
        assert_eq!(layers[1].pixels, vec![7, 8, 9, 0, 1, 2]);
        assert_eq!(layers[1].color_counts.len(), 6);
    }

    #[test]
    fn test_flatten_layers() {
        let content = "0222112222120000".to_string();
        let layers = extract_layers(&content, 2, 2);
        let flattened = flatten_layers(&layers);

        assert_eq!(flattened, vec![0, 1, 1, 0]);
    }
}