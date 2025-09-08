use rand::Rng;
use crate::organismo::Organismo;
use std::collections::HashMap;

pub struct Simulador {
    pub poblacion: Vec<Box<dyn Organismo>>,
    pub eventos: Vec<String>, // ← lista de eventos del día
}

impl Simulador {


    
    pub fn new() -> Self {
        Simulador { poblacion: Vec::new(), eventos: Vec::new() }
    }

    

    pub fn agregar(&mut self, organismo: Box<dyn Organismo>) {
        self.poblacion.push(organismo);
    }



    pub fn contar<T: 'static>(&self) -> usize {
        self.poblacion
            .iter()
            .filter(|o| o.as_any().is::<T>())
            .count()
    }

    pub fn simular_dia(&mut self, dia: u32) {
        let mut rng = rand::thread_rng();
        let mut nuevos = Vec::new();
        let mut muertos = 0;

        // --- ENVEJECER Y REPRODUCCIÓN ---
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

        self.poblacion.extend(nuevos);

        // --- CAZA DEL ÚNICO LOBO ---
        let indice_lobo = self.poblacion.iter().position(|org| org.nombre() == "Lobo" && org.esta_vivo());

                if let Some(_i) = indice_lobo {
            // recolectamos presas disponibles (≥100 días)
            let presas: Vec<(usize, f64)> = self.poblacion.iter().enumerate()
                .filter(|(_, org)| org.nombre() != "Lobo" && org.edad() >= 50)
                .map(|(j, org)| (j, org.peso()))
                .collect();

            if !presas.is_empty() {
                // buscamos la presa más pesada
                let max_peso = presas.iter().map(|(_, p)| *p).fold(0.0, f64::max);
                let candidatas: Vec<_> = presas.into_iter().filter(|(_, p)| *p == max_peso).collect();

                // si hay empate, elegimos al azar
                let (idx, _) = candidatas[rng.gen_range(0..candidatas.len())];

                // eliminamos la presa y guardamos su peso
                let presa = self.poblacion.remove(idx);
                let peso_presa = presa.peso();

                // ahora sí, buscar al lobo de nuevo y darle la comida
               if let Some(lobo) = self.poblacion.iter_mut().find(|o| o.nombre() == "Lobo" && o.esta_vivo()) {
            // ← bloque a insertar:
            if let Some(l) = lobo.as_any_mut().downcast_mut::<crate::lobo::Lobo>() {
                l.agregar_comida(peso_presa);
                let mensaje = format!("🐺 El lobo cazó un {} de {:.2} kg", presa.nombre(), peso_presa);
                println!("{}", mensaje);       // consola
                self.eventos.push(mensaje);   // UI
            }
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
            println!("\n🐾 {} (total: {})", especie, lista.len());
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


