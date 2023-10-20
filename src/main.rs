#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::Error;
use std::io::Result;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
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

    let age:i32 = 8;
    if(age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age  == 21) || (age == 50) {
        println!("Important Birthday");
    } else if  age >= 65 {
        println!("Important Birthday"); 
    } else {
        println!("Not an Important Birthday");
    }
    //turnary operator

    
    
}
