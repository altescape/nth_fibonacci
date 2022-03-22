use dialoguer::{theme::ColorfulTheme, Input, Confirm};

fn main() {
    clearscreen::clear().unwrap();

    loop {
        // Prompt input
        let num: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Input a number:")
            .interact_text()
            .unwrap();

        clearscreen::clear().unwrap();

        let num: u32 = num.trim().parse().expect("A number is required");

        clearscreen::clear().unwrap();

        println!("Next number in Fibonacci sequence is {}", next_fibonacci(num));

        // Prompt continue or quit
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Continue or quit?")
            .default(true)
            .wait_for_newline(true)
            .interact_opt()
            .unwrap()
            {
                Some(true) => {
                    clearscreen::clear().unwrap();
                    continue;
                },
                Some(false) => {
                    clearscreen::clear().unwrap();
                    break;
                },
                None => break,
            }
    }
}

fn next_fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    return next_fibonacci(n - 1) + next_fibonacci(n - 2);
}

// This function returns the wrong value, @todo: see why
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
