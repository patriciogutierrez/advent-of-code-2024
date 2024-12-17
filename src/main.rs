use std::{array, collections::HashMap, fs::File, io::{ BufReader, Read}, iter::Map};

fn main() {
    day1_2();
}


fn day1_1(){
    static FILE_PATH: &str = "src/day1_1.txt";
    let file = File::open(FILE_PATH).unwrap();
    let mut reader = BufReader::new(file);
    let mut array1: Vec<i32> = vec![];
    let mut array2: Vec<i32> = vec![];

    let mut line = String::new(); 
    reader.read_to_string(&mut line).unwrap();

    line.split("\r\n").collect::<Vec<&str>>().into_iter().for_each(|line|{
        let values = line.split("   ").collect::<Vec<&str>>();
        array1.push(values[0].parse::<i32>().unwrap());
        array2.push(values[1].parse::<i32>().unwrap());
    });
    array1.sort();
    array2.sort();

    let mut total = 0;
    for i in 0..array1.len() {
        total += array1[i].abs_diff(array2[i]);
    }
    println!("{:?}",total);
}

fn day1_2(){
    static FILE_PATH: &str = "src/day1_1.txt";
    let file = File::open(FILE_PATH).unwrap();
    let mut reader = BufReader::new(file);
    let mut array1:HashMap<i32,i32> = Default::default();
    let mut array2:HashMap<i32,i32> = Default::default();
    let mut line = String::new(); 
    reader.read_to_string(&mut line).unwrap();

    line.split("\r\n").collect::<Vec<&str>>().into_iter().for_each(|line|{
        let values = line.split("   ").collect::<Vec<&str>>();
        let  value= values[0].parse::<i32>().unwrap();
        array1.insert(value,  array1.get(&value).cloned().unwrap_or_default() + 1);

        let  value= values[1].parse::<i32>().unwrap();
        array2.insert(value,  array2.get(&value).cloned().unwrap_or_default() + 1);
    });
   
    let mut total = 0;
    for k in array1.keys(){
        if !array2.contains_key(k) {continue};
        total += array1.get(k).unwrap() * k * array2.get(k).unwrap();
    }
    println!("{:?}", total);

}