use std::io::Write;

fn main() {
    let mut cwd = std::env::current_dir().unwrap();

    loop {
        let mut input = String::new();

        print!("{} > ", cwd.display());

        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).expect("err");

        let parts = input
            .trim()
            .split(" ")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        if parts.len() == 0 {
            continue;
        }

        match parts[0].as_str() {
            "glungus" => {
                println!("🐱");
            }
            "exit" => break,
            "" => {}
            "ls" => {
                let dir_contents = std::fs::read_dir(&cwd).unwrap();
                for entry in dir_contents {
                    println!(
                        "{}",
                        entry
                            .expect("reason")
                            .path()
                            .iter()
                            .last()
                            .unwrap()
                            .display()
                    );
                }
            }
            "cd" => {
                let new_path = cwd.join(parts.get(1).expect("usage: cd [dir]"));

                if !new_path.is_dir() {
                    panic!("it isnt a directory!!");
                }
                std::env::set_current_dir(&new_path).unwrap();

                cwd = new_path;
            }

            "clear" => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                std::io::stdout().flush().unwrap();
            }

            _ => {
                println!("rsh: command not found: {input}");
            }
        }
    }
}
