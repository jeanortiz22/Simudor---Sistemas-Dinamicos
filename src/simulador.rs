use rand::Rng;
use crate::organismo::Organismo;
use std::collections::{HashMap, VecDeque};

pub struct Simulador {
    // Un vector que puede contener cualquier tipo que implemente `Organismo`.
    pub poblacion: Vec<Box<dyn Organismo>>,
    // Una cola para almacenar los eventos de caza y mostrarlos en la UI.
    pub eventos: VecDeque<String>,
}

impl Simulador {
    pub fn new() -> Self {
        Simulador {
            poblacion: Vec::new(),
            eventos: VecDeque::with_capacity(30),
        }
    }

    pub fn registrar_evento(&mut self, mensaje: String) {
        // Agrega un evento a la cola, eliminando el más antiguo si la capacidad está llena.
        if self.eventos.len() == 30 {
            self.eventos.pop_front(); 
        }
        self.eventos.push_back(mensaje); 
    }

    // Añade un nuevo organismo a la población del simulador.
    pub fn agregar(&mut self, organismo: Box<dyn Organismo>) {
        self.poblacion.push(organismo);
    }

    // Cuenta cuántos organismos de un tipo específico (`T`) hay en la población.
    pub fn contar<T: 'static>(&self) -> usize {
        self.poblacion
            .iter()
            .filter(|o| o.as_any().is::<T>())
            .count()
    }

    // Simula un día completo en el ecosistema.
    pub fn simular_dia(&mut self, dia: u32) {
        let mut rng = rand::thread_rng();
        let mut nuevos = Vec::new();
        let mut muertos = 0;

        // ENVEJECER Y REPRODUCCIÓN
        self.poblacion.retain_mut(|org| {
            org.envejecer();
            nuevos.extend(org.reproducirse(&mut rng));

            if org.esta_vivo() {
                true
            } else {
                muertos += 1;
                println!("☠️ {} murió a los {} días (peso final {:.2})",
                         org.nombre(), org.edad(), org.peso());
                false
            }
        });

        self.poblacion.extend(nuevos); // Agrega todas las nuevas crías.

        // CAZA DEL LOBO SEGÚN RESERVA
        // Busca el lobo para que cace.
        if let Some(lobo_idx) = self.poblacion.iter().position(|org| org.nombre() == "Lobo" && org.esta_vivo()) {
            if let Some(lobo_ref) = self.poblacion[lobo_idx].as_any().downcast_ref::<crate::lobo::Lobo>() {
                if lobo_ref.necesita_cazar() && !lobo_ref.tiene_reserva_llena() {
                    // Selecciona las presas disponibles (cabras y conejos maduros).
                    let presas_cabras: Vec<(usize, f64)> = self.poblacion.iter().enumerate()
                        .filter(|(_, org)| org.nombre() == "Cabra" && org.edad() >= 50)
                        .map(|(j, org)| (j, org.peso()))
                        .collect();

                    let presas_conejos: Vec<(usize, f64)> = self.poblacion.iter().enumerate()
                        .filter(|(_, org)| org.nombre() == "Conejo" && org.edad() >= 50)
                        .map(|(j, org)| (j, org.peso()))
                        .collect();

                    // Elige la especie a cazar de forma aleatoria si ambas están disponibles.
                    let especie_objetivo = if !presas_cabras.is_empty() && !presas_conejos.is_empty() {
                        if rng.gen_bool(0.5) { "Cabra" } else { "Conejo" }
                    } else if !presas_cabras.is_empty() {
                        "Cabra"
                    } else if !presas_conejos.is_empty() {
                        "Conejo"
                    } else {
                        ""
                    };

                    if especie_objetivo != "" {
                        let presas = if especie_objetivo == "Cabra" { presas_cabras } else { presas_conejos };

                        //  Buscar la presa más pesada 
                        let max_peso = presas.iter().map(|(_, p)| *p).fold(0.0, f64::max);
                        let candidatas: Vec<_> = presas.into_iter().filter(|(_, p)| *p == max_peso).collect();

                        //  Si hay empate, elegir al azar 
                        let (idx, _) = candidatas[rng.gen_range(0..candidatas.len())];

                        //  Eliminar presa y darle comida al lobo 
                        let presa = self.poblacion.remove(idx);
                        let peso_presa = presa.peso();

                        // Se busca al lobo de nuevo con una referencia mutable para actualizar su reserva.
                        if let Some(lobo_mut) = self.poblacion.iter_mut().find(|o| o.nombre() == "Lobo" && o.esta_vivo()) {
                            if let Some(l) = lobo_mut.as_any_mut().downcast_mut::<crate::lobo::Lobo>() {
                                l.agregar_comida(peso_presa);
                                let mensaje = format!(" El lobo cazó un {} de {:.2} kg", presa.nombre(), peso_presa);
                                println!("{}", mensaje);
                                self.registrar_evento(mensaje);
                            }
                        }
                    }
                } else {
                    println!(" El lobo no cazó hoy (reserva: {:.2})", lobo_ref.reserva);
                }
            }
        }

        // --- REPORTE ---
        let mut especies: HashMap<&str, Vec<&Box<dyn Organismo>>> = HashMap::new();
        for org in &self.poblacion {
            especies.entry(org.nombre()).or_default().push(org);
        }

        println!("\n--- Día {} ---", dia);
        println!("Murieron {} organismos en este día.", muertos);

        for (especie, lista) in &especies {
            println!("\n {} (total: {})", especie, lista.len());
            for (i, org) in lista.iter().enumerate() {
                println!(
                    "   ID {} -> (edad: {} días, peso: {:.2})",
                    i,
                    org.edad(),
                    org.peso()
                );
            }
        }

        println!("\n📊 Total población: {}\n", self.poblacion.len());
    }
}
