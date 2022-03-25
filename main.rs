extern crate argparse;
extern crate rand;

use argparse::{ArgumentParser, Store};
use rand::prelude::*;

fn main() {
    let mut length = 16;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Generate a safe password.");

        ap.refer(&mut length).add_option(
            &["--length", "-L"],
            Store,
            "Set desired password's length",
        );

        ap.parse_args_or_exit();
    }
    let generated_password = generate_password(length);
    println!("Generated password: {}", generated_password);
}

fn generate_password(length: i32) -> String {
    let characters = String::from("!#$%&()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_abcdefghijklmnopqrstuvwxyz{|}~");
    let mut result = String::from("");

    let mut rng = rand::thread_rng();
    while (result.len() as i32) < length {
        let random: f64 = rng.gen();
        let selected_float = random * (characters.len() as f64);
        let selected_index = selected_float.floor() as usize;

        result.push(characters.chars().nth(selected_index).unwrap());
    }

    result
}
