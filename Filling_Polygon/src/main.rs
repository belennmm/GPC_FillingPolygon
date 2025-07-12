use raylib::prelude::*;

fn main() {
    // dimensiones de la ventanaa
    let screen_width = 800;
    let screen_height = 600;

    // empieza la ventana
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Rellenando Polígonos - Belén <3")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // ----------- Polígono 1: -------------- 
        let polygon1 = vec![
            Vector2::new(165.0, 380.0),
            Vector2::new(185.0, 360.0),
            Vector2::new(180.0, 330.0),
            Vector2::new(207.0, 345.0),
            Vector2::new(233.0, 330.0),
            Vector2::new(230.0, 360.0),
            Vector2::new(250.0, 380.0),
            Vector2::new(220.0, 385.0),
            Vector2::new(205.0, 410.0),
            Vector2::new(193.0, 383.0),
        ];

        // se dibuja el polígono como una línea 
        for i in 0..polygon1.len() {
            let current = polygon1[i];
            let next = polygon1[(i + 1) % polygon1.len()];
            d.draw_line(current.x as i32, current.y as i32, next.x as i32, next.y as i32, Color::BLACK);
        }

        // 
    }
}
