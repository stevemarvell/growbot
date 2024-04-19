
#[derive(Debug)]
pub(crate) struct World {
    age: u32,
    population: Vec<Genotype>,
}

impl World {
    pub(crate) fn new(size: usize, genotype_length: usize) -> Self {
        let population = (0..size)
            .map(|_| Genotype::new(genotype_length))
            .collect::<Vec<Genotype>>();

        Self { age: 0, population }
    }
}

#[derive(Debug)]
struct Genotype {
    genes: Vec<i32>,
}

use rand::prelude::*;

impl Genotype {
    fn new(length: usize) -> Self {
        let mut rng = rand::thread_rng();
        let genes = (0..length)
            .map(|_| rng.gen_range(0..64))
            .collect::<Vec<i32>>();

        Self { genes }
    }
}
