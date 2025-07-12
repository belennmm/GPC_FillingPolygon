use raqote::*;
use image::{RgbaImage, Rgba};

fn main() {

    // tamaño de la imagen
    let width = 800;
    let height = 600;

    // crea el target para pintar los dibujitos  
    let mut dt = DrawTarget::new(width, height);

    let mut pb = PathBuilder::new();

    // -------------- Polígono 1: ---------------------
    // puntos dados
    let points = vec![
        (165.0, 380.0),
        (185.0, 360.0),
        (180.0, 330.0),
        (207.0, 345.0),
        (233.0, 330.0),
        (230.0, 360.0),
        (250.0, 380.0),
        (220.0, 385.0),
        (205.0, 410.0),
        (193.0, 383.0),
    ];

    // path 
    pb.move_to(points[0].0, points[0].1);
    for &(x, y) in &points[1..]{
         pb.line_to(x, y) ;
    }
    pb.close(); // → esto cierra la figura


     let path = pb.finish();

    // relleno amarillo
    let color = Source::Solid(SolidSource{
        r: 242,
        g: 245,
        b: 39,
        a: 255,
    }) ;
    dt.fill(&path, &color, &DrawOptions::new());

    // orilla blanca
    let stroke_color = Source::Solid(SolidSource{
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    }) ;


    // dibuja la línea de contorno

    let stroke_style = StrokeStyle{
        width: 2.0,
        ..StrokeStyle::default()
    };
    dt.stroke(&path, &stroke_color, &stroke_style, &DrawOptions::new());



    // ---------- Exportar la imagen: ------------

    // se obtienen los datos que pusimos en el DrawTarget
    let data = dt.get_data();
// se crea la imagen 
    let mut image = RgbaImage::new(width as u32, height as u32);
    
    // aquí se pasan los pixeles para pasarlos a la imagen
    for y in 0..height{
        for x in 0..width{
            let pixel = data[(y as usize) * (width as usize) + (x as usize)];
            let r = ((pixel >> 16) & 0xff) as u8;
            let g = ((pixel >> 8) & 0xff) as u8;
            let b = (pixel & 0xff) as u8;
            let a = ((pixel >> 24) & 0xff) as u8;

            image.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
        }
    }
        // se guarda en png
    image.save("output.png").unwrap();
}
