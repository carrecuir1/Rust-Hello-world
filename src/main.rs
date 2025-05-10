// I put this just to remove warnings as this is a sandbox to learn rust
#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    arrays_example();
}

fn hello_word() {
    println!("What is your name ?");
    let mut name = String::new();
    let greeting= "Nice to meet you";

    io::stdin().read_line(&mut name)
        .expect("Didn't receive Input");

    println!("Hello, {}! {}", name.trim_end(), greeting);
}

fn shadowing_example() {
    const ONE_MILL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age="25";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MILL);
}

fn data_types_example() {
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);

    // bool example
    // Adding _ at the start of the variable, 
    // will make the compiler ignore the fact it's not being used
    let _is_true = true;

    //char
    let _my_grade = 'A';
}

fn math_example() {
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("{} + {} = {}", num_3, num_4, num_3 + num_4);
    println!("{} - {} = {}", num_3, num_4, num_3 - num_4);
    println!("{} * {} = {}", num_3, num_4, num_3 * num_4);
    println!("{} / {} = {}", num_3, num_4, num_3 / num_4);
    println!("{} % {} = {}", num_3, num_4, num_3 % num_4);
    num_3 += 1;
}

fn random_example() {
    let random_num = rand::rng().random_range(1..101);
    println!("Random : {}", random_num);
}

fn condition_example() {
    // let age = 18;
    let age = rand::rng().random_range(1..101);
    let voting_age = 18;
    println!("Age is {}", age);

    // if (age >= 1) && (age <= 18){
    //     println!("Important Birthday");
    // } else if (age >= 21) && (age <= 50) {
    //     println!("Also an Important Birthday");
    // } else if age >= 65 {
    //     println!("Older, But still an Important Birthday");
    // } else
    // {
    //     println!("Sorry... not an important birthday");
    // }

    match age {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };

    // let can_vote = if age >= 18{
    //     true
    // }
    // else{
    //     false
    // };
    // println!("Can Vote : {}", can_vote);

    match age.cmp(&voting_age){
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };

}

fn arrays_example() {
    let arr = [1,2,3,4,5,6,7,8,9];
    println!("1st : {}", arr[0]);
    println!("Length : {}", arr.len());

    let mut loop_idx = 0;

    println!("Normal loop");
    loop {
        // even number
        if arr[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        
        if arr[loop_idx] == 9{
           break; 
        }
        println!("Val : {}", arr[loop_idx]);
        loop_idx+=1;
    }
    loop_idx = 0;

    println!("While loop");
    while loop_idx < arr.len(){
        println!("Arr : {}", arr[loop_idx]);
        loop_idx+=1;
    }   
    
    println!("For loop");
    loop_idx = 0;
    for val in arr.iter() {
        println!("Val : {}", val);
    }
}