use rand::rngs::ThreadRng;
use std::any::Any;

//LO ingreso para manejar sexos entre las presas
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Sexo {
    Macho,
    Hembra,
}

pub trait Organismo {
    fn envejecer(&mut self);
    fn reproducirse(&self, rng: &mut ThreadRng) -> Vec<Box<dyn Organismo>>;
    fn peso(&self) -> f64;
    fn esta_vivo(&self) -> bool;
    fn nombre(&self) -> &str;
    fn edad(&self) -> u32;



    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}


impl dyn Organismo {
    pub fn is<T: 'static>(&self) -> bool {
        self.as_any().is::<T>()
    }
}
