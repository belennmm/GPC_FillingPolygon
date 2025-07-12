use raqote::*;
use image::{RgbaImage, Rgba};

fn main() {

    // tamaño de la imagen
    let width = 800;
    let height = 600;

    // crea el target para pintar los dibujitos  
    let mut dt = DrawTarget::new(width, height);

    let mut pb = PathBuilder::new();

    
        // se guarda en png
    image.save("output.png").unwrap();
}
