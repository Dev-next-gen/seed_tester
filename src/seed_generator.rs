use rand::Rng;

pub fn generate_seeds(num_seeds: usize) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let mut seeds = Vec::new();

    for _ in 0..num_seeds {
        seeds.push(rng.gen::<u64>());  // Génère un seed de 64 bits
    }

    seeds
}
