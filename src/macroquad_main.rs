// src/macroquad_main.rs
mod organismo;
mod simulador;
mod cabra;
mod conejo;
mod lobo;
mod modelo;

use macroquad::prelude::*;
use ::rand::Rng;          // 👈 usa el trait desde el rand del Cargo.toml
use ::rand::rngs::ThreadRng; // 👈 para thread_rng

 // usar rand del Cargo.toml
use simulador::Simulador;
use cabra::Cabra;
use conejo::Conejo;
use lobo::Lobo;

// Función para poblar la simulación con organismos iniciales
// Función para poblar la simulación (espera ThreadRng explícito)
// Función para poblar la simulación
fn poblar(sim: &mut Simulador, rng: &mut ThreadRng) {
    for _ in 0..100 {
        sim.agregar(Box::new(Cabra::new_random(rng)));
    }
    for _ in 0..100 {
        sim.agregar(Box::new(Conejo::new_random(rng)));
    }
    sim.agregar(Box::new(Lobo::new()));
}




#[macroquad::main("Ecosistema - Barras")]
async fn main() {
    // -----------------------------------------------------------------
    // Configuración inicial
    // -----------------------------------------------------------------
    let mut sim = Simulador::new();
    let mut rng = ::rand::thread_rng();

    poblar(&mut sim, &mut rng);

    // -----------------------------------------------------------------
    // Estados de la UI / animación
    // -----------------------------------------------------------------
    let mut dia: u32 = 0;
    let mut timer = 0.0_f32;
    let intervalo_dia = 0.6_f32; // segundos por día

    let mut paused = false;

    // valores animados de altura
    let mut display_cabras: f32 = 0.0;
    let mut display_conejos: f32 = 0.0;
    let mut display_lobos: f32 = 0.0;

    loop {
        let dt = get_frame_time();

        // teclas
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            sim = Simulador::new();
            poblar(&mut sim, &mut rng);
            dia = 0;
            display_cabras = 0.0;
            display_conejos = 0.0;
            display_lobos = 0.0;
            timer = 0.0;
        }

        // avanzar simulación
        if !paused {
            timer += dt;
            if timer >= intervalo_dia {
                timer -= intervalo_dia;
                dia = dia.saturating_add(1);
                sim.simular_dia(dia);
            }
        }

        // contar especies
        let num_cabras: f32 = sim.contar::<Cabra>() as f32;
        let num_conejos: f32 = sim.contar::<Conejo>() as f32;
        let num_lobos: f32 = sim.contar::<Lobo>() as f32;

        // máximo para escalar
        let max_poblacion = num_cabras.max(num_conejos).max(num_lobos).max(1.0);

        // dimensiones
        let screen_w = screen_width();
        let screen_h = screen_height();
        let base_y = screen_h - 90.0;
        let max_bar_h = screen_h - 220.0;
        let scale = max_bar_h / max_poblacion;

        let bar_w = 140.0;
        let gap = 40.0;
        let total_w = 3.0 * bar_w + 2.0 * gap;
        let start_x = (screen_w - total_w) / 2.0;

        let x_cabras = start_x;
        let x_conejos = start_x + (bar_w + gap);
        let x_lobos = start_x + 2.0 * (bar_w + gap);

        // animación suave
        let anim_speed = 6.0;
        let target_cabras = num_cabras * scale;
        let target_conejos = num_conejos * scale;
        let target_lobos = num_lobos * scale;

        display_cabras += (target_cabras - display_cabras) * (1.0 - (-anim_speed * dt).exp());
        display_conejos += (target_conejos - display_conejos) * (1.0 - (-anim_speed * dt).exp());
        display_lobos += (target_lobos - display_lobos) * (1.0 - (-anim_speed * dt).exp());

        // -----------------------------------------------------------------
        // Dibujo
        // -----------------------------------------------------------------
        clear_background(WHITE);

        draw_text(
            &format!("Ecosistema - Día: {}{}", dia, if paused { " (PAUSADO)" } else { "" }),
            24.0,
            36.0,
            30.0,
            BLACK,
        );
        draw_text("Población por especie (barras)", 24.0, 64.0, 20.0, DARKGRAY);

        draw_line(start_x - 20.0, base_y, start_x + total_w + 20.0, base_y, 2.0, BLACK);

        // Cabras
        draw_rectangle(x_cabras, base_y - display_cabras, bar_w, display_cabras, RED);
        draw_text(&format!("Cabras\n{}", num_cabras as i32), x_cabras + 12.0, base_y + 30.0, 22.0, BLACK);

        // Conejos
        draw_rectangle(x_conejos, base_y - display_conejos, bar_w, display_conejos, GREEN);
        draw_text(&format!("Conejos\n{}", num_conejos as i32), x_conejos + 12.0, base_y + 30.0, 22.0, BLACK);

        // Lobos
        draw_rectangle(x_lobos, base_y - display_lobos, bar_w, display_lobos, BLUE);
        draw_text(&format!("Lobos\n{}", num_lobos as i32), x_lobos + 12.0, base_y + 30.0, 22.0, BLACK);

        // info
        draw_text(&format!("Total población: {}", sim.poblacion.len()), 24.0, screen_h - 20.0, 20.0, DARKBLUE);
        draw_text("Space: Pausa/Reanuda    R: Reiniciar", screen_w - 420.0, screen_h - 20.0, 18.0, DARKGRAY);

        



        // Buscar lobo y mostrar alimento
        if let Some(lobo) = sim.poblacion.iter().find_map(|o| o.as_any().downcast_ref::<Lobo>()) {
            draw_text(
                &format!("🐺 Alimento del lobo: {:.1} kg", lobo.reserva),
                24.0,
                100.0,
                24.0,
                DARKBLUE,
            );
        }


        let mut y_eventos = 150.0;

        // Dibujar eventos de caza del lobo
        for evento in &sim.eventos {
            draw_text(evento, 24.0, y_eventos, 20.0, DARKBLUE);
            y_eventos += 24.0; // espacio entre líneas
        }


        next_frame().await;

    }


    
}
