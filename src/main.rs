mod ga;

use ga::World;

fn main() {
    let population_size = 10;
    let genotype_length = 10;
    let world = World::new(population_size, genotype_length);
    println!("{:?}", world);
}
