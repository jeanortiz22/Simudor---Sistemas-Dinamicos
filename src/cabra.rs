use crate::organismo::{Organismo, Sexo};
use crate::modelo::gompertz;
use rand::Rng;
use std::any::Any;

pub struct Cabra {
    pub edad: u32,
    pub peso: f64,
    pub sexo: Sexo,
    pub viva: bool,
}

impl Cabra {
    pub fn new_random(rng: &mut rand::rngs::ThreadRng) -> Self {
        let sexo = if rng.gen_bool(0.5) { Sexo::Macho } else { Sexo::Hembra };
        Cabra { edad: 0, peso: 0.0, sexo, viva: true }
    }

    pub fn new_con_sexo(sexo: Sexo) -> Self {
        Cabra { edad: 0, peso: 0.0, sexo, viva: true }
    }
}

impl Organismo for Cabra {
    fn envejecer(&mut self) {
        if !self.viva { return; }
        self.edad += 1;
        self.peso = gompertz(self.edad as f64, 60.0, 0.01, 150.0);

        // Muerte por vejez
        if self.edad >= 365 * 1 {
            self.viva = false;
            return;
        }

        // Probabilidad diaria de enfermarse y morir
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.005) { // 0.5% de probabilidad
            self.viva = false;
        }
    }
    
    //Manejo de forma aleatoria el tema de machos y hembras al ingresar el valor inicial 
    //Añado que solo hembras reproduzcan, prob de nacimiento 50/50, añado el tema de probabilidad de nacimiento segun a cantidad en el main 
    fn reproducirse(&self, rng: &mut rand::rngs::ThreadRng) -> Vec<Box<dyn Organismo>> {
        let mut crias: Vec<Box<dyn Organismo>> = Vec::new();
        if !self.viva { return crias; }

        if self.edad > 365 && self.sexo == Sexo::Hembra && rng.gen_bool(0.01) {
            let cantidad = rng.gen_range(1..=2);
            for _ in 0..cantidad {
                crias.push(Box::new(Cabra::new_random(rng))); //Ingreso a la box o vec en el head 
            }
        }

        crias //LO retorno 
    }

    fn peso(&self) -> f64 { self.peso }
    fn esta_vivo(&self) -> bool { self.viva }
    fn nombre(&self) -> &str { "Cabra" }
    fn edad(&self) -> u32 { self.edad }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
