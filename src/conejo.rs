use crate::organismo::{Organismo, Sexo};
use crate::modelo::gompertz;
use rand::Rng;
use std::any::Any;

pub struct Conejo {
    pub edad: u32,
    pub peso: f64,
    pub sexo: Sexo,
    pub viva: bool,
}

impl Conejo {

    // Crea un nuevo conejo con una edad inicial de 0, peso 0.0,
    // y un sexo aleatorio (50% macho, 50% hembra). 
    pub fn new_random(rng: &mut rand::rngs::ThreadRng) -> Self {
        let sexo = if rng.gen_bool(0.5) { Sexo::Macho } else { Sexo::Hembra };
        Conejo { edad: 0, peso: 0.0, sexo, viva: true }
    }
    // Crea un nuevo conejo con un sexo específico.
    pub fn new_con_sexo(sexo: Sexo) -> Self {
        Conejo { edad: 0, peso: 0.0, sexo, viva: true }
    }   
}
//Implementacion del trait por conejo
impl Organismo for Conejo {
    fn envejecer(&mut self) {
        if !self.viva { return; }
        self.edad += 1;
        // El peso del conejo se actualiza cada día con la función Gompertz.
        self.peso = gompertz(self.edad as f64, 2.0, 0.05, 50.0);

        // Muerte por vejez
        if self.edad >= 365 *2 {
            self.viva = false;
            return;
        }

        // Probabilidad diaria de enfermarse y morir
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.01) { // 1% de probabilidad
            self.viva = false;
        }
    }
    
    //Reproduccion de conejo
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
