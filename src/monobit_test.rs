use rand::Rng;

pub fn run_monobit_test() {
    let mut rng = rand::thread_rng();
    let total_bits = 1_000_000;
    let ones = (0..total_bits).filter(|_| rng.gen::<bool>()).count();
    let proportion = ones as f64 / total_bits as f64;
    println!("Test de Monobit : Proportion de '1' = {}", proportion);
}
