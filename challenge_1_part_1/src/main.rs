use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

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
    // No idea why we need & yet 
    // let f = calc_dist(list1, list2);
    // println!("DIST: {f}");
    let sim_score = calc_similarity_score(list1, list2);
    println!("SIM SCORE: {sim_score}");
}


fn calc_similarity_score(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    // No matter what we need a set to match. Something like a python dict.
    // to do the matching:
    // 1. brute force it which is just taking the set and scanning the second list for appearances.
    //    calc a score and add it to a cum sum
    // 2. That's lots of wasted work, when instead we could use bin search. 
    // The first is going to be m * n where m is the number of unique nums in list 1 and n is list
    // list2
    // The second is going to be m * log(n) same deal, but binary search is faster than scanning 
    // The fastest way, since both are sorted, is to increment up each using two pointers
    // That way is just max(m,n) depending on which is longer.
    // NVM we can't guarentee sortedness from the input so we'll get fucked doing that by the sort
    // time 
    // still a way to do it linearly using memory
    // create a freq count for each num in list2
    // then iterate through list1 and if that num is in the freq count calc the similarity and add
    // return similarity. That's just max(n,m) with max(n,m) extra memory, add to a set to prevent double
    // counting 
    // Maybe we can do it without the extra mem? 
    // That's super anoying BUT... 
    // We can do some leetcode array manipulation BS to embed the counts in the existing array.
    // But then we can't do lookups on the array when it comes time to do the matching 
    // Nah it isn't possible b/c the input nums are unbounded. (Maybe not but too lazy to analyize)
    // Just goint wo go with the simplest approach that's linear time.
    //
    
    let mut unique = HashSet::new();
    for item in list1 {
        unique.insert(item);
    }

    let freq = freq_count_in_dict(list2);
    let mut total = 0;
    for id in &unique {
        total += id * *freq.get(id).unwrap_or(&0);
    }

    return total;
}

fn freq_count_in_dict(mut list: Vec<i32>) -> HashMap<i32, i32> {
    let mut freq = HashMap::new();
    for item in list {
        let count = freq.entry(item).or_insert(0);
        *count += 1;
    }
    return freq;
}

fn freq_count_in_vec(mut list: Vec<i32>) -> Vec<i32> {
    // Convert a vec to encode freq info in vec
    // E.g. [1,2,3,4,1,2,3] -> [1,2,2,2,3,2,4,1]
    return Vec::<i32>::new();
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
