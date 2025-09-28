use std::io::Write;

#[derive(serde::Deserialize, Debug, Default)]
struct Config {
    path: Vec<String>,
    prompt: Option<String>,
}

fn main() {
    let home_dir = std::env::home_dir().expect("no home dir");
    let config_file = std::fs::read_to_string(home_dir.join(".rshrc.toml")).unwrap_or_default();
    let config: Config = toml::from_str(&config_file).unwrap();

    let mut cwd = std::env::current_dir().unwrap();

    loop {
        let mut input = String::new();

        if let Some(prompt) = &config.prompt {
            print!(
                "{} > ",
                prompt
                    .replace("$u", &whoami::username())
                    .replace("$h", &whoami::fallible::hostname().unwrap_or_default())
                    .replace("$w", &cwd.display().to_string())
            );
        } else {
            print!("{} > ", cwd.display());
        }

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

            "exit" => {
                println!("exit");
                break;
            }

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

            "" => {}

            _ => {
                let mut found = false;
                for p in &config.path {
                    let cmd_path = std::path::Path::new(p).join(&parts[0]);
                    if cmd_path.is_file() {
                        found = true;
                        let mut cmd = std::process::Command::new(cmd_path);
                        if parts.len() > 1 {
                            cmd.args(&parts[1..]);
                        }
                        cmd.current_dir(&cwd);
                        let status = cmd.status().expect("failed to execute process");
                        if !status.success() {
                            eprintln!("process exited with non-zero status");
                        }
                        break;
                    }
                }
                if !found {
                    eprintln!("rsh: command not found: {}", parts[0]);
                }
            }
        }
    }
}
