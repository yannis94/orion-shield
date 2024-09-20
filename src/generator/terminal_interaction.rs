use super::{engine::Command, password::PasswordConfig};
use std::io;

pub fn print_title() {
    print!("+-+-+-+-+ ORION SHIELD +-+-+-+-+\n");
    print!("Password generator\n\n");
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
}

pub fn print_menu() {
    println!("\n+-+-+-+-+ Menu +-+-+-+-+\n");
    print_command();
}

pub fn print_config(cfg: String) {
    println!("Current configuration:");
    println!("{}", cfg);
}

fn print_command() {
    println!("[M]odify config\n[G]enerate\n[Q]uit");
    print!("Choice > ");
}

fn update_cfg() -> Command {
    let mut new_cfg: PasswordConfig = (8, false, false, false);
    let mut invalid_cfg: bool = true;

    while invalid_cfg {
        let mut input = String::new();

        println!("Password length > ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to get input");

        match input.trim().parse::<u8>() {
            Ok(l) => new_cfg.0 = l,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }

        let mut input = String::new();
        println!("uppercase [y, n] > ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to get input");

        if input.trim().to_lowercase() == "y" {
            new_cfg.1 = true;
        } else {
            new_cfg.1 = false;
        }

        let mut input = String::new();
        println!("special characters [y, n] > ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to get input");

        if input.trim().to_lowercase() == "y" {
            new_cfg.2 = true;
        } else {
            new_cfg.2 = false;
        }

        let mut input = String::new();
        println!("with digits [y, n] > ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to get input");

        if input.trim().to_lowercase() == "y" {
            new_cfg.3 = true;
        } else {
            new_cfg.3 = false;
        }

        invalid_cfg = false;
    }
    // get cfg from user
    return Command::UpdateConfig(new_cfg);
}

pub fn get_user_command() -> Command {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to get input");

    let cmd = input.trim().to_lowercase();

    if cmd == "m" {
        return update_cfg();
    } else if cmd == "g" {
        return Command::Generate;
    } else if cmd == "q" {
        return Command::Quit;
    } else {
        println!("command \"{}\" does not exist", cmd);
        return Command::GetConfig;
    }
}
