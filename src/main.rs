use rand::Rng;
use weighted_rand::builder::{WalkerTableBuilder, NewBuilder};
use std::{iter, env::{self}};

const FOREST: [char; 12] = ['🌲', ' ', '🌳',  '🍄', '🌱','🌿', '🍃','🐻', '🍀', '🪵', '🌷', '🐸'];
const WEIGHTS: [f32; 12] = [0.45, 0.35, 0.1, 0.02, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01];

fn plant_forest(len: usize) -> String {
    let builder = WalkerTableBuilder::new(&WEIGHTS);
    let wa_table = builder.build();
    let mut rng = rand::thread_rng();
    let one_char = || {
        let idx = wa_table.next_rng(&mut rng);
        let c = FOREST[idx];
        if [' ','🪵'].contains(&c) {
            let mut c_e = c.to_string();
            c_e.push(' ');
            c_e
        } else {
            c.to_string()
        }
    };
    iter::repeat_with(one_char).take(len).collect()
}

fn burn_forest(forest: &str) -> String {
    let mut rng = rand::thread_rng();
    forest.chars().map(|x| {
        if rng.gen_ratio(1, 4) {
            x
        } else {
            match x {
                '🌲' => '🔥',
                '🌳' => '🔥',
                '🐻' => '💀',
                '🐸' => '💀',
                ' ' => ' ',
                _ => '💥',
            }
        }
    }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && args[1] == "plant" {
        match args[2].parse() {
            Ok(nbr) => println!("{}", plant_forest(nbr)),
            Err(_) => println!("Invalid number passed as argument!"),
        }
    } else if args.len() == 3 && args[1] == "burn" {
        println!("{}", burn_forest(&args[2]));
    } else {
        println!("Invalid arguments!");
    }
}
