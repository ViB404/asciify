use image::imageops::{grayscale, FilterType};
use image::open;

fn main() {
    let medium = [" ", "░", "▒", "▓", "█"];
    let light = [" ", ".", ":", "-", "=", "+", "*", "#", "%", "@"];
    let dark = [
        " ", ".", "'", "`", "^", "\"", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+", "_",
        "-", "?", "]", "[", "}", "{", "1", ")", "(", "|", "\\", "/", "t", "f", "j", "r", "x", "n",
        "u", "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q", "p",
        "d", "b", "k", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "@", "$",
    ];

    let palette = dark;

    let image = open("testing.jpeg").unwrap().resize(100, 100, FilterType::Nearest).into_rgba8();
    
    let grayscale_img = grayscale(&image);

    let (width, height) = grayscale_img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel_value = grayscale_img.get_pixel(x, y)[0];

            let index = (pixel_value as usize * (palette.len() - 1)) / 255;

            print!("{}", palette[index]);
        }
        println!();
    }
}
