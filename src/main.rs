mod organismo;
mod simulador;
mod cabra;
mod conejo;
mod lobo;
mod modelo;

use simulador::Simulador;
use cabra::Cabra;
use conejo::Conejo;
use lobo::Lobo;

fn main() {
    let mut sim = Simulador::new();
    let mut rng = rand::thread_rng();

    // Población inicial configurable
    let num_cabras_iniciales = 100;
    let num_conejos_iniciales = 100;
    let num_lobos_iniciales = 1; // 🔹 ahora varios lobos iniciales

    // Cabras iniciales
    for _ in 0..num_cabras_iniciales {
        sim.agregar(Box::new(Cabra::new_random(&mut rng)));
    }

    // Conejos iniciales
    for _ in 0..num_conejos_iniciales {
        sim.agregar(Box::new(Conejo::new_random(&mut rng)));
    }

    // Lobos iniciales
    for _ in 0..num_lobos_iniciales {
        sim.agregar(Box::new(Lobo::new())); 
    }

    // Simulación de varios días
    for dia in 1..=300{ 
        sim.simular_dia(dia);
    }
}
