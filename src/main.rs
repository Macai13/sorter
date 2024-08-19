mod sorting;

use std::{thread, time::{Duration, Instant}};
use colored::Colorize;

fn main()
{
    loop 
    {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let mut vec: Vec<i64> = vec![];

        println!("{}", "Vector size: ".purple());
        let size = int_input();

        println!("{}", "\nWould you prefer to fill the vector with random numbers [0/1]?".yellow());

        match int_input()
        {
            0 => 
            {
                println!("{}", "\nVector numbers: ".purple());

                for _ in 0..size 
                {
                    vec.push(int_input());
                }
            }

            1 => 
            {
                use rand::prelude::*;

                let mut rng = rand::thread_rng();
                
                for _ in 0..size 
                {
                    vec.push(rng.gen_range(0..20_000));
                }
            }

            _ => 
            {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

                println!("{}", "Invalid input.".red());
                thread::sleep(Duration::from_millis(1200));

                continue;
            }
        }

        println!("{}", "\nWhat sorting would you like to do?".yellow());
        println!("{}", "1. Bubble sort".purple());
        println!("{}", "2. Optimized bubble sort".purple());
        println!("{}", "3. Quick sort".purple());
        println!("{}", "4. Selection sort".purple());
        println!("{}", "5. Bogo sort".purple());
        println!("{}", "6. Radix sort".purple());
        println!("{}", "0. Exit".red());

        let choice = int_input();

        if size <= 1
        {
            println!("Nothing to sort!");
            thread::sleep(Duration::from_millis(1200));

            continue;
        }

        println!("{} {:?}", "\nunsorted:".red(), vec);

        let now: Instant;
        let elapsed: Duration;
        let iterations: u64;

        match choice 
        {
            1 => 
            {
                use sorting::bubble_sort_no_flag;

                now = Instant::now();
                iterations = bubble_sort_no_flag::sort(&mut vec);
                elapsed = now.elapsed();
            },

            2 => 
            {
                use sorting::bubble_sort_with_flag;

                now = Instant::now();
                iterations = bubble_sort_with_flag::sort(&mut vec);
                elapsed = now.elapsed();
            },

            3 => 
            {
                use sorting::quick_sort;

                let vec_size = vec.len();
                now = Instant::now();
                iterations = quick_sort::sort(&mut vec, 0, (vec_size - 1) as isize);
                elapsed = now.elapsed();
            },

            4 =>
            {
                use sorting::selection_sort;

                now = Instant::now();
                iterations = selection_sort::sort(&mut vec);
                elapsed = now.elapsed();
            },

            5 =>
            {
                use sorting::bogo_sort;

                now = Instant::now();
                iterations = bogo_sort::sort(&mut vec);
                elapsed = now.elapsed();
            },

            6 =>
            {
                use sorting::radix_sort;

                now = Instant::now();
                iterations = radix_sort::sort(&mut vec);
                elapsed = now.elapsed();
            }

            0 =>
            {
                println!("Program end.");
                std::process::exit(0);
            },

            _ =>
            {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

                println!("{}", "Invalid input.".red());
                thread::sleep(Duration::from_millis(1200));

                continue;
            },
        }

        print_result(&mut vec, iterations);
        println!("Time elapsed: {}", format!("{:.2?}", elapsed).blue());

        println!("\nPress enter to go again...");
        input();
    }
}

fn int_input() -> i64
{
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line)
    {
        Ok(_) => (),
        Err(_) => std::process::exit(1),
    }

    if let Some('\n') = line.chars().next_back() 
    {
        line.pop();
    }

    if let Some('\r') = line.chars().next_back() 
    {
        line.pop();
    }

    match line.parse::<i64>()
    {
        Ok(v) => v,
        Err(_) => -1,
    }
}

fn input() -> String
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    line
}

fn print_result(sorted_vec: &mut Vec<i64>, iterations: u64)
{
    println!("{} {:?}", "sorted:".green(), sorted_vec);
    println!("Sorting finished after {} iterations.", format!("{:?}", iterations).blue());
}