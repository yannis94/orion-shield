use orion_shield::generator::{Password, PasswordConfig};

fn main() {
    let cfg: PasswordConfig = (32, true, true, true);
    let pwd: Password = Password::new(cfg);

    for _i in 0..10 {
        print!("new pwd: {}\n", pwd.generate());
    }
}
