mod sorting;
use sorting::{bubble_sort_no_flag, bubble_sort_with_flag, quick_sort};
use colored::Colorize;

fn main()
{
    loop 
    {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal

        let mut vec: Vec<i64> = vec![];

        println!("{}", "Vector size: ".purple());
        let size = int_input();

        println!("{}", "\nVector numbers: ".purple());

        for _ in 0..size 
        {
            vec.push(int_input());
        }

        println!("{}", "\nWhat sorting would you like to do?".yellow());
        println!("{}", "1. Bubble sort".purple());
        println!("{}", "2. Optimized bubble sort".purple());
        println!("{}", "3. Quick sort".purple());
        println!("{}", "0. Exit".red());

        let choice = int_input();

        println!("{} {:?}", "\nunsorted:".red(), vec);

        match choice 
        {
            1 => 
            {
                let (sorted_vec, iterations) = bubble_sort_no_flag::sort(vec);

                print_result(sorted_vec, iterations);
            },

            2 => 
            {
                let (sorted_vec, iterations) = bubble_sort_with_flag::sort(vec);

                print_result(sorted_vec, iterations);
            },

            3 => 
            {
                let vec_size = vec.len();
                let iterations = quick_sort::sort(&mut vec, 0, vec_size - 1);

                print_result(vec, iterations);
            },

            0 => 
            {
                println!("Program end.");
                std::process::exit(0);
            },

            _ => continue,
        }

        println!("\nPress any key to go again...");
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

    line.parse::<i64>().unwrap()
}

fn input() -> String
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    line
}

fn print_result(sorted_vec: Vec<i64>, iterations: u64)
{
    println!("{} {:?}", "sorted:".green(), sorted_vec);
    println!("finished after {} iterations.", format!("{:?}", iterations).blue());
}