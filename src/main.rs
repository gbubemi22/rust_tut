#![allow(unused)]

use rand::prelude;
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::Error;
use std::io::Result;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn get_sum2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {

println!("{}", get_sum2(5,4));


    // println!("What is your name?");
    // let mut name = String::new();
    // let greeting: &str = "Nice to meet you";
    // io::stdin().read_line(&mut name).expect("Failed to read input");

    // println!("Hello {}!  {}", name.trim_end(), greeting);

    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Failed to parse age as number");

    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    //DATA TYPES

    // Unsigned integer : u8, u16, u32, u64, u128, usize
    //    Signed integer: i8, i16, i32, i64, i128,

    println!("max u32: {}", u32::MAX);
    println!("max u32: {}", u64::MAX);
    println!("max u32: {}", usize::MAX);
    println!("max u32: {}", u128::MAX);
    println!("max u32: {}", f32::MAX);
    println!("max u32: {}", f64::MAX);

    //Boolean

    let is_true: bool = true;
    let my_grade: char = 'A';

    //Math

    let num_1: f32 = 1.111111111111111;

    println!("f32: {}", num_1 + 0.1111111111111111);

    let num_2: f64 = 1.1111111111111111;

    println!("f64: {}", num_2 + 0.1111111111111111);

    let mut num3: u32 = 5;
    let num4: u32 = 4;

    println!("5 + 4 = {}", num3 + num4);
    println!("5 - 4 = {}", num3 - num4);
    println!("5 * 4 = {}", num3 * num4);
    println!("5 / 4 = {}", num3 / num4);
    println!("5 % 4 = {}", num3 % num4);

    num3 += 1;

    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);

    //condition

    let age: i32 = 0;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }
    //match

    let age2: i32 = 8;
    match age2 {
        1..=18 => println!("important Birthtday"),
        21 | 50 => println!("important Birthtday"),
        65..=i32::MAX => println!("important Birthtday"),
        _ => println!("Not an important Birthtday"),
    };

    let my_age: i32 = 18;
    let voting_age: i32 = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("can't  Vote"),
        Ordering::Greater => println!("can  Vote"),
        Ordering::Equal => println!("You gained the right to Vote"),
    }

    //Arrays

    let arr_1: [i32; 4] = [1, 2, 3, 4];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    //loop an array

    let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx: usize = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("val  : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    let arr_3: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx1: usize = 0;

    while loop_idx1 < arr_3.len() {
        println!("Arr  : {}", loop_idx1);
        loop_idx1 += 1;
    }
    //tuple
    let my_tuple: (u8, String, f64) = (47, "Dreake".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;

    println!("Age : {}", v1);
    let mut st1: String = String::new();
    st1.push('A'); // Append 'A'
    st1.push_str(" words"); // Append " words"

    for word in st1.split_whitespace() {
        println!("{}", word);
    }
   let st2: String = st1.replace("A", "Another");
   println!("{}", st2);


   let st3: String = String::from("x,r,t,b,h,k,k,a,m,c");

   let mut v1: Vec<char> = st3.chars().collect();
   v1.sort();
   v1.dedup();
   for ch in &v1 {
    println!("{}", ch);
}

   let st4: &str = "Random string";
   let mut st5: String = st4.to_string();
   println!("{}", st5);

   let byte_arr1: &[u8] = st5.as_bytes();
   let st6: &str = &st5[0..6];
   println!("String lenght : {}", st6.len());

  let st6: String = String::from("Just some");
  let st7: String = String::from(" words");
  let st8: String = st6 + &st7;
  println!( "{}", st8);

//enum

enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

impl Day {
    fn is_weekend(&self) -> bool {
        match self {
            Day::Saturday | Day::Sunday => true,
            _ => false,
        }
    }
}

let today:Day = Day::Monday;
match today {
  Day::Monday => println!("Everyone hets Monday"),
  Day::Tuesday => println!("Donut day"),
  Day::Wednesday => println!("Hump day"),
  Day::Thursday => println!("Pay day"),
  Day::Friday => println!("Almost Weekend"),
  Day::Saturday => println!("Weekend"),
  Day::Sunday => println!("Weekend"),
}


println!("Is today the weekend  {}", today.is_weekend());

let result = get_sum2(5, 7);
println!("The sum is: {}", result);


}
