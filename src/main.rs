mod mathengine;
use mathengine::core::*;

use std::io::{self, Write};

fn main() {
    let mut calc = Calculator::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading from STDIN");
        if user_input.trim().to_lowercase() == "quit" {
            break;
        }
        let output = calc.eval(&user_input);
        match output {
            Ok(eval_result) => match eval_result {
                EvalResult::Answer(ans) => println!("  {}", ans),
                EvalResult::Feedback(fb) => println!("  {}", fb),
            },
            Err(e) => println!("{} {}", "!", e),
        }
    }
}
