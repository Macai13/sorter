mod lib;
use lib::bubble_sort;

fn main()
{
    loop 
    {
        let mut vec: Vec<i64> = vec![];

        println!("Vector size: ");
        let size = int_input();

        println!("Vector numbers: ");

        for i in 0..size 
        {
            vec.push(int_input());
        }

        println!("unsorted: {:?}", vec);

        let sorted_vec = bubble_sort::sort(vec);

        println!("sorted: {:?}", sorted_vec);
        println!("\nPress any key to go again...");
        input();
    }
}

fn int_input() -> i64 
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

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