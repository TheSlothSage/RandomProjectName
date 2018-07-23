extern crate rand;
#[macro_use] extern crate text_io;
use rand::Rng;
use std::{io, io::prelude::*, fs::File, vec, io::BufReader};

fn main() {

    //Full path to text file with adjectives and adverbs
    
    println!("Please enter the adjective text file: ");
    
    let ad_path: String = read!();
    
    println!("Please enter the noun text file: ");
    
    let noun_path: String = read!();
    
    let ad_file = File::open(ad_path).expect("unable to open");
    let noun_file = File::open(noun_path).expect("unable to open");
    let ad_file = BufReader::new(ad_file);
    let noun_file = BufReader::new(noun_file);
    let mut nounv : std::vec::Vec<String> = std::vec::Vec::new();
    let mut adv : std::vec::Vec<String> = std::vec::Vec::new();
    
    for x in ad_file.lines()
    {
        let l = x.unwrap();
        adv.push(l);
    }

    for x in noun_file.lines()
    {
        let l = x.unwrap();
        nounv.push(l);
    }

    let number_of_adstrings = adv.len();
    let number_of_nounstrings = nounv.len();

    let random_number1 = rand::thread_rng().gen_range(0,number_of_adstrings);    
    let random_number2 = rand::thread_rng().gen_range(0,number_of_nounstrings);    
    println!("{}:{}", adv[random_number1], nounv[random_number2]);

}//main
