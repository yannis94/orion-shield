use std::process;

use orion_shield::generator::{engine::Engine, terminal_interaction};

fn main() {
    terminal_interaction::print_title();
    terminal_interaction::print_menu();

    let mut engine = Engine::new();

    loop {
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
