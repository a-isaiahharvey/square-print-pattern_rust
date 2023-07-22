use std::io::{self, Write};

fn print_error_message() {
    println!();
    println!("Number must NOT contain spaces.");
    println!("Number must NOT contain letters.");
    println!("Number must NOT contain symbols.");
    println!("Number must NOT be a decimal number.");
    println!("Number must NOT be a negative integer.");
    println!("Number must NOT be an even integer.");
    println!("Number must NOT be blank.");
    println!();
}

fn validate_input(input: String, input_num: &mut usize) -> bool {
    let trimmed_input = input.trim();

    if trimmed_input.starts_with('-') || trimmed_input.starts_with('\0') {
        print_error_message();
        return false;
    } else {
        for i in 0..trimmed_input.len() {
            if trimmed_input.chars().nth(i).unwrap().is_numeric() {
            } else {
                print_error_message();
                return false;
            }
        }
    }

    if trimmed_input.ends_with('1')
        || trimmed_input.ends_with('3')
        || trimmed_input.ends_with('5')
        || trimmed_input.ends_with('7')
        || trimmed_input.ends_with('9')
    {
        *input_num = trimmed_input.parse::<usize>().unwrap();
        true
    } else {
        false
    }
}

fn get_user_number() -> usize {
    loop {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();

        let mut input_num: usize = 0;
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let flag = validate_input(input, &mut input_num);
        if flag {
            return input_num;
        }
    }
}

fn initial_square_array(ary: &mut [Vec<char>], odd_int: usize, input: &char) {
    for chars in ary.iter_mut().take(odd_int) {
        for c in chars.iter_mut().take(odd_int) {
            *c = *input;
        }
    }
}

fn fill_square_array(ary: &mut [Vec<char>], odd_int: usize, index: u32, input: &char) {
    for i in (index as usize..=(odd_int / 2)).step_by(2) {
        for j in i..=(odd_int - 1) - i {
            ary[i][j] = *input;
            ary[(odd_int - 1) - i][j] = *input;
            ary[j][i] = *input;
            ary[j][(odd_int - 1) - i] = *input;
        }
    }
}

fn print_square_array(ary: &mut [Vec<char>], odd_int: usize) {
    for chars in ary.iter().take(odd_int) {
        for c in chars.iter().take(odd_int) {
            print!("{}", c);
            print!(" ");
        }
        println!();
    }
}

fn print_pattern(ary: &mut [Vec<char>], odd_int: usize) {
    if odd_int % 4 == 1 {
        initial_square_array(ary, odd_int, &' ');
        fill_square_array(ary, odd_int, 0, &'X');
    } else {
        initial_square_array(ary, odd_int, &'X');
        fill_square_array(ary, odd_int, 1, &' ');
    }
    print_square_array(ary, odd_int)
}

fn main() {
    let odd_int = get_user_number();
    let ary = &mut vec![vec![' '; odd_int]; odd_int];
    println!();
    print_pattern(ary, odd_int);
}
