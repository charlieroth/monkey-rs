use monkey_rs::lexer::Lexer;
use monkey_rs::token::Token;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    // #[cfg(feature = "with-file-history")]
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history file.");
    // }

    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let mut lexer = Lexer::new(&line);
                loop {
                    let next = lexer.next();
                    if next == Token::Eof {
                        break;
                    } else {
                        println!("{:?}", next);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // #[cfg(feature = "with-file-history")]
    // rl.save_history("history.txt");

    Ok(())
}
