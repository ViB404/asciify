use crate::image_ascii::format_ascii;
use image::load_from_memory;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use vid2img::FileSource;

pub fn video_to_frame() {
    let file_path = Path::new("testing_video.mp4");

    // change it according to your terminal size
    let frame_size = (120, 34);
    let frame_source = FileSource::new(&file_path, frame_size).unwrap();

    println!("decoding video.. please wait");
    let mut ascii_frames = Vec::new();

    for frame in frame_source.into_iter() {
        if let Ok(Some(png_img_data)) = frame {
            if let Ok(dynamic_image) = load_from_memory(&png_img_data) {
                let grayscale_image = dynamic_image.into_luma8();

                ascii_frames.push(format_ascii(grayscale_image));
            }
        }
    }

    sleep(Duration::from_secs(2));

    clearscreen::clear().expect("failed to clear screen");
    print!("\x1B[?25l"); 

    for frame_string in ascii_frames {
        print!("\x1B[1;1H");
        print!("{}", frame_string);
        
        // so it doesn't look choppy
        sleep(Duration::from_millis(33)); 
    }

    print!("\x1B[?25h"); 
}