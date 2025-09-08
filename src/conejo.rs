use crate::organismo::{Organismo, Sexo};
use crate::modelo::gompertz;
use rand::Rng;
use std::any::Any;

pub struct Conejo {
    pub edad: u32,
    pub peso: f64,
    pub sexo: Sexo, //Viene el enum que se encuentra en organismos, para no repetir lo mismo en todad las presas
    pub viva: bool,
}

impl Conejo {

    //Hago la implementacion de los sexos por especie, para manejar de forma real la reproducccion 
    pub fn new_random(rng: &mut rand::rngs::ThreadRng) -> Self {
        let sexo = if rng.gen_bool(0.5) { Sexo::Macho } else { Sexo::Hembra };
        Conejo { edad: 0, peso: 0.0, sexo, viva: true }
    }

    pub fn new_con_sexo(sexo: Sexo) -> Self {
        Conejo { edad: 0, peso: 0.0, sexo, viva: true }
    }
}

impl Organismo for Conejo {
    fn envejecer(&mut self) {
        if !self.viva { return; }
        self.edad += 1;
        self.peso = gompertz(self.edad as f64, 2.0, 0.05, 50.0);

        // Muerte por vejez
        if self.edad >= 365 * 1 {
            self.viva = false;
            return;
        }

        // Probabilidad diaria de enfermarse y morir
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.01) { // 2% de probabilidad
            self.viva = false;
        }
    }
    
    //Revisar cabra, aplique lo mismo 
    fn reproducirse(&self, rng: &mut rand::rngs::ThreadRng) -> Vec<Box<dyn Organismo>> {
        let mut crias: Vec<Box<dyn Organismo>> = Vec::new();
        if !self.viva { return crias; }

        if self.edad > 90 && self.sexo == Sexo::Hembra && rng.gen_bool(0.1) {
            let cantidad = rng.gen_range(1..=4);
            for _ in 0..cantidad {
                crias.push(Box::new(Conejo::new_random(rng)));
            }
        }

        crias
    }

    fn peso(&self) -> f64 { self.peso }
    fn esta_vivo(&self) -> bool { self.viva }
    fn nombre(&self) -> &str { "Conejo" }
    fn edad(&self) -> u32 { self.edad }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
