use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Hello, world! I'm getting the file at {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have read the file bruh");
    let (mut list1, mut list2) = split_input(&contents);
    // println!("with text: \n {contents}");

    let first_vec_elem = list1[0];
    let second_vec_elem = list2[0];
    println!("list1: {first_vec_elem}");
    println!("list2: {second_vec_elem}");
    list1.sort();
    list2.sort();

    let first_vec_elem = list1[0];
    let second_vec_elem = list2[0];

    let last_vec1_elem = list1.last().unwrap();
    let last_vec2_elem = list2.last().unwrap();
    println!(" sorted list1: {first_vec_elem}");
    println!(" sorted list2: {second_vec_elem}");
    println!(" sorted list1: {last_vec1_elem}");
    println!(" sorted list2: {last_vec2_elem}");
    let f = calc_dist(list1, list2);
    println!("FINAL: {f}");
}


fn calc_dist(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();
    let mut cum: i32 = 0;
    let mut iter = list1.iter().zip(list2.iter());
    for (elem1, elem2) in iter {
        let dist = elem1 - elem2;
        cum += dist.abs();
    }
    

    return cum;
}

fn split_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    // Split on newline_split
    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];

    let newline_split: Vec<&str> = input.split("\n").collect();
    for pair in newline_split {
        println!("found pair: {pair}");
        if let Some((elem1, elem2)) = pair.split_once("   ") {
            println!("elem1: \'{elem1}\'");
            println!("elem2: \'{elem2}\'");
            vec1.push(elem1.parse::<i32>().unwrap());
            vec2.push(elem2.parse::<i32>().unwrap());
        } else {
            println!("BAD BAD BAD");
        }

    }

    return (vec1, vec2)
}
