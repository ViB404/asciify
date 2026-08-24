use image::ImageBuffer;
pub fn format_ascii(grayscale_image_buffer: ImageBuffer<image::Luma<u8>, Vec<u8>>) -> String {
    let _medium = [" ", "░", "▒", "▓", "█"];
    let _light = [" ", ".", ":", "-", "=", "+", "*", "#", "%", "@"];
    let dark = [
        " ", ".", "'", "`", "^", "\"", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+", "_",
        "-", "?", "]", "[", "}", "{", "1", ")", "(", "|", "\\", "/", "t", "f", "j", "r", "x", "n",
        "u", "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q", "p",
        "d", "b", "k", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "@", "$",
    ];
    
    let palette = dark;

    let (width, height) = grayscale_image_buffer.dimensions();

    let mut frame_str = String::with_capacity((width * height + height) as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel_value = grayscale_image_buffer.get_pixel(x, y)[0];
            let index = (pixel_value as usize * (palette.len() - 1)) / 255;

            frame_str.push_str(palette[index]);
        }
        frame_str.push('\n');
    }

    frame_str
}
