use rand::Rng;

mod hello;

fn main() {
    let pseudo_rand = rand::thread_rng().gen_range(1..11);
    println!("Pseudo-Rand: {}", pseudo_rand);
    hello::print_hello();
}
