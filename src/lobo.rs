use crate::organismo::Organismo;
use crate::modelo::gompertz;
use rand::Rng;
use std::any::Any;

pub struct Lobo {
    pub edad: u32,
    pub peso: f64,      // peso corporal (crece con gompertz)
    pub reserva: f64,   // comida acumulada
    pub vivo: bool,
    pub nivel_minimo: f64,
    pub nivel_optimo: f64,
}

impl Lobo {
    pub fn new() -> Self {
        Lobo {
            edad: 0,
            peso: 0.0,
            reserva: 1500.0,
            vivo: true,
            nivel_minimo: 5.0,   // necesita mínimo 2 kg/día
            nivel_optimo: 10.0,   // idealmente 5 kg/día
        }
    }

    /// El lobo consume de su reserva
fn alimentarse(&mut self) {
    if !self.vivo { return; }

    if self.reserva < self.nivel_minimo {
        println!("☠️ El lobo murió de hambre con {:.2} kg en reserva.", self.reserva);
        self.vivo = false;
    } else {
        // consumo aleatorio entre mínimo y óptimo
        let mut rng = rand::thread_rng();
        let consumo_deseado = rng.gen_range(self.nivel_minimo..=self.nivel_optimo);
        let consumo = consumo_deseado.min(self.reserva);

        self.reserva -= consumo;
        println!(
            "🐺 El lobo consumió {:.2} kg de su reserva (quedan {:.2})",
            consumo, self.reserva
        );
    }
}


    /// Permite aumentar la reserva (ej: cuando caza en el simulador)
    pub fn agregar_comida(&mut self, cantidad: f64) {
        if !self.vivo { return; }
        self.reserva += cantidad;
        println!("🍖 El lobo almacenó {:.2} kg de comida (total reserva: {:.2})", cantidad, self.reserva);
    }
}

impl Organismo for Lobo {
    fn envejecer(&mut self) {
        if !self.vivo { return; }
        self.edad += 1;
        self.peso = gompertz(self.edad as f64, 50.0, 0.008, 300.0);
        self.alimentarse(); // cada día consume de la reserva
    }

    fn reproducirse(&self, rng: &mut rand::rngs::ThreadRng) -> Vec<Box<dyn Organismo>> {
        if self.edad > 500 && rng.gen_bool(0.005) {
            vec![Box::new(Lobo::new())]
        } else {
            vec![]
        }
    }

    fn peso(&self) -> f64 { self.peso }
    fn esta_vivo(&self) -> bool { self.vivo }
    fn nombre(&self) -> &str { "Lobo" }
    fn edad(&self) -> u32 { self.edad }

        fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

}
