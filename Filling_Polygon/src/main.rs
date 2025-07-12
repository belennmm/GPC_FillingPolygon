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


    // -------------- Polígono 2: ---------------------

    // otro path para el 2
    let mut pb2 = PathBuilder::new();

    // puntos dados del 2
    let polygon2 = vec![
        (321.0, 335.0),
    (288.0, 286.0),
        (339.0, 251.0),
        (374.0, 302.0),
    ];


    pb2.move_to(polygon2[0].0, polygon2[0].1);
    for &(x, y) in &polygon2[1..]{
         pb2.line_to(x, y);
    }
    pb2.close();

    let path2 = pb2.finish();

    // relleno azul
    let color2 = Source::Solid(SolidSource{
        r: 39,
        g: 82,
        b: 200,
        a: 255,
    }) ;
    dt.fill(&path2, &color2, &DrawOptions::new());

    // orilla blanca
    let stroke_color2 = Source::Solid(SolidSource{
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    });
    // contorno
    let stroke_style2 = StrokeStyle{
         width: 2.0,
        ..StrokeStyle::default()
    };
    dt.stroke(&path2, &stroke_color2, &stroke_style2, &DrawOptions::new());


    // -------------- Polígono 3: ---------------------
    // path para el 3
    let mut pb3 = PathBuilder::new();

    // puntos del triángulo dados
    let polygon3 = vec![
        (377.0, 249.0),
        (411.0, 197.0),
         (436.0, 249.0),
    ];

    pb3.move_to(polygon3[0].0, polygon3[0].1);
    for &(x, y) in &polygon3[1..]{
        pb3.line_to(x, y);
    }
     pb3.close();


    let path3 = pb3.finish();

    // color rojo
    let color3 = Source::Solid(SolidSource{
        r: 227,
        g: 42,
        b: 42,
        a: 255,
    });

        dt.fill(&path3, &color3, &DrawOptions::new());

    
    // orilla blanca 
    let stroke_color3 = Source::Solid(SolidSource{
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    });

    let stroke_style3 = StrokeStyle{
         width: 2.0,
        ..StrokeStyle::default()
    };

    dt.stroke(&path3, &stroke_color3, &stroke_style3, &DrawOptions::new());


    // -------------- Polígono 4: tiene un agujero: ---------------------
    /*para el 4 */
    let mut pb4 = PathBuilder::new();

    // puntos dados para el cuatro (tipo tetera)
    let polygon4 = vec![
        (413.0, 177.0), (448.0, 159.0), (502.0, 88.0), (553.0, 53.0),
        (535.0, 36.0), (676.0, 37.0), (660.0, 52.0), (750.0, 145.0),
        (761.0, 179.0), (672.0, 192.0), (659.0, 214.0), (615.0, 214.0),
         (632.0, 230.0), (580.0, 230.0), (597.0, 215.0), (552.0, 214.0),
        (517.0, 144.0), (466.0, 180.0),
    ];

    // parte de afuera 
    pb4.move_to(polygon4[0].0, polygon4[0].1);
    for &(x, y) in &polygon4[1..]{
        pb4.line_to(x, y);
    }

    pb4.close();

    // ---------- Este debe ser el polígono 5
    // Es el agujero 
    let hole = vec![
        (682.0, 175.0),  (708.0, 120.0),  (735.0, 148.0), (739.0, 170.0),
    ];

    // se pone en el sentido opuesto para ser un agujero
    pb4.move_to(hole[0].0, hole[0].1);
    for &(x, y) in hole[1..].iter().rev() {  // al opuesto
        pb4.line_to(x, y);
    }

    pb4.line_to(hole[0].0, hole[0].1); /* se cierra */
    pb4.close();

    let path4 = pb4.finish();

    // color verde 
    let color4 = Source::Solid(SolidSource{
        r: 80 ,
        g: 210,
        b: 53,
        a: 255,
    });
    dt.fill(&path4, &color4, &DrawOptions::new());

    // blanco 
    let stroke_color4 = Source::Solid(SolidSource{
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    });

    // pinta el contorno
    let stroke_style4 = StrokeStyle {
        width: 2.0,
        ..StrokeStyle::default()
    };

     dt.stroke(&path4, &stroke_color4, &stroke_style4, &DrawOptions::new());




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
    image.save("polygon.png").unwrap();
}
