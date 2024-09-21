use clap::Parser;
use std::process;

use orion_shield::generator::{engine::Engine, terminal_interaction};

/// Orion-Shield is a password generator.
#[derive(Parser)]
struct Cli {
    /// Use interactive mode (app)
    #[arg(short = 'i', long = "interactive")]
    interactive_mode: bool,
    /// Give a password length config
    #[arg(short = 'l', long = "length", default_value_t = 20)]
    length: u8,

    /// Give a password with uppercase config [default: false]
    #[arg(short = 'u', long = "uppercase")]
    has_upper: bool,

    /// Give a password with special characters config [default: false]
    #[arg(short = 'c', long = "spec-chars")]
    has_spec: bool,

    /// Give a password with digit config [default: false]
    #[arg(short = 'd', long = "digit")]
    has_digit: bool,
}

fn main() {
    let args = Cli::parse();
    if args.interactive_mode {
        run_app();
    } else {
        run(args);
    }
}

fn run(args: Cli) {
    let mut engine = Engine::new();
    let cmd = engine.exec(orion_shield::generator::engine::Command::UpdateConfig((
        args.length,
        args.has_upper,
        args.has_spec,
        args.has_digit,
    )));

    match cmd {
        Ok(_res) => print!(""),
        Err(e) => {
            println!("unable to compute password config: {}", e);
            process::exit(1);
        }
    }

    let cmd = engine.exec(orion_shield::generator::engine::Command::Generate);

    match cmd {
        Ok(pwd) => println!("\nGenerated password > {}\n", pwd),
        Err(e) => println!("{}", e),
    }
}

fn run_app() {
    terminal_interaction::print_title();
    terminal_interaction::print_menu();
    let mut engine = Engine::new();

    loop {
        engine.get_cfg();
        let c = terminal_interaction::get_user_command();
        match engine.exec(c) {
            Ok(res) => {
                terminal_interaction::clear_screen();
                println!("{}", res);
                terminal_interaction::print_menu()
            }
            Err(e) => {
                println!("{}", e);
                process::exit(1)
            }
        }
    }
}
