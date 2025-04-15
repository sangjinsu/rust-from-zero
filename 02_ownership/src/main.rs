mod move_ownership;
mod copy;
mod borrow;
mod lifetime;

fn main() {
    println!("\n== Move Example ==");
    move_ownership::run();

    println!("\n== Copy Example ==");
    copy::run();

    println!("\n== Borrow Example ==");
    borrow::run();

    println!("\n== Lifetime Example ==");
    lifetime::run();
}
