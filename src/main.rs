use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};


fn main() {
    loop {
        print!("~λ ");
        stdout().flush().unwrap();


        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();


        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;


        while let Some(command) = commands.next()  {
            let mut input_parts = command.trim().split_whitespace();
            let command = input_parts.next().unwrap();
            let arguments = input_parts;


            match command {
                "exit" => return,
                "cd" => {
                    let new_directory =
                        arguments.peekable().peek().map_or("/", |x| *x);

                    let root = Path::new(new_directory);
                    if let Err(error) = env::set_current_dir(&root) {
                        eprintln!("{}", error);
                    }

                    previous_command = None;
                },
                command => {
                    let stdin =
                        previous_command.map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };


                    let output =
                        Command::new(command)
                            .args(arguments)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn();


                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }

    }
}
