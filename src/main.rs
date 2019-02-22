use std::io;
use std::io::Write;

//Functions that take a Vector, and populate them to capacity
//with the fibonacci series.
mod fibonacci_sequences {

    //Iteratively generate the fibonacci series
    pub fn iterative_get_series(series_vector: &mut Vec<u128>) {

        //Find out the capacity of the vector passed
        //And calculate that many elements.
        let vector_length: usize = series_vector.capacity();

        //No elements need to be calculated.
        if vector_length == 0 {

            return;
        }

        series_vector.push(0);

        //The first element is 0, no elements
        //need to be calculated.
        if vector_length == 1 {

            return;
        }

        series_vector.push(1);

        //Calculate the fibbonaci numbers
        //up to the capacity of the passed vector.
        for x in 2..vector_length {

            let first_val: u128 = series_vector[x - 2];

            let second_val: u128 = series_vector[x - 1];

            series_vector.push(first_val + second_val);
        }
    }

    //Seeded recursive function for calculating fibonacci numbers.
    fn recursive_get_series_seeded(length: usize, series_vector: &mut Vec<u128>, 
                                   last_index: usize) {

        if length == 0 {

            return;
        }

        let first_val: u128 = series_vector[last_index - 2];

        let second_val: u128 = series_vector[last_index - 1];

        series_vector.push(first_val + second_val);

        recursive_get_series_seeded(length - 1, series_vector, last_index + 1);
    }

    //Recursively calculate the fibonacci series up to the
    //capacity of the passed vector.
    pub fn recursive_get_series(series_vector: &mut Vec<u128>) {

        let series_length: usize = series_vector.capacity();

        if series_length == 0 {

            return;
        }

        series_vector.push(0);

        if series_length == 1 {

            return;
        }

        series_vector.push(1);

        recursive_get_series_seeded(series_length - 2, series_vector, 2);
    }
}

fn main() {

    let mut valid_series_length: bool = false;

    //The number of fibonacci numbers to calculate
    let mut series_length: u64 = 0;

    //ask user for input until valid input is entered.
    while !valid_series_length {

        print!("Enter the number of elements to calculate => ");

        //Make sure the user gets the prompt
        //before trying to read.
        io::stdout().flush().expect("Unable to flush stdout");

        //String to hold what the user types in.
        let mut user_response = String::new();

        //Read the user response into the string.
        io::stdin().read_line(&mut user_response).expect("Unable to read line from stdout.");

        series_length = match user_response.trim().parse() {
            
            //The user entered in a valid number
            Ok(number) =>  {

                valid_series_length = true;
                
                number
            },

            //The input is not a number.
            Err(_) => {
                
                0
            }
        };
    }

    //Create a vector with the correct size for the series.
    let mut iterative_fibonacci_series: Vec<u128> = Vec::with_capacity(series_length as usize);

    let mut recursive_fibonacci_series: Vec<u128> = Vec::with_capacity(series_length as usize);

    fibonacci_sequences::recursive_get_series(&mut iterative_fibonacci_series);

    fibonacci_sequences::iterative_get_series(&mut recursive_fibonacci_series);

    println!("Iterative Series Calculation:");
    
    for x in iterative_fibonacci_series {

        print!("{} ", x);
    }

    println!();

    println!("Recursive Series Calculation:");

    for x in recursive_fibonacci_series {

        print!("{} ", x);
    }

    println!();
}
