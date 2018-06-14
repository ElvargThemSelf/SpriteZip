//std crates
extern crate core;
extern crate time;

//external crates
extern crate image;
extern crate brotli;
extern crate walkdir;

//standard uses
use std::io::{Write};
use std::fs::File;
use time::PreciseTime;

//non-standard use
use image::{RgbaImage, GenericImage};
use walkdir::WalkDir;

// TODO: crop diff'd images  so that not so much data needs to be compressed?

/*fn SubtractTwoImages(img1_dyn : &image::DynamicImage, img2_dyn : &image::DynamicImage, debug_mode : bool)
{
}*/

fn main() {
    let canvas_width = 3000;
    let canvas_height = 3000;

    let debug_mode = true;

    let f = File::create(["compressed_images", ".brotli"].concat()).expect("Cannot create file");

    let mut compressor = brotli::CompressorWriter::new(
    f,
    4096,
    9,//11, //9 seems to be a good tradeoff...changing q doesn't seem to make much diff though?
    22);

    let mut canvas = RgbaImage::new(canvas_width, canvas_height);

    println!("Begin scanning for images");

    //TODO: check that input_images directory exists before scanning it.
    //TODO: check each image's color type as subtracting a RGB image from an RGBA image shouldn't work.
    // see: println!("{:?}", img.color());

    let test_iter = WalkDir::new("input_images");
    let mut count = 0;
    for entry in test_iter
    {
        let ent = entry.unwrap();

        let file_name_no_ext = ent.path().file_stem().unwrap().to_str().unwrap();//ent.path();

        if ent.file_type().is_dir() {
            continue;
        }

        let save_path = [file_name_no_ext, ".png"].concat();
        println!("Will save image to: {}", save_path);

        if count == 0 //first image
        {
            println!("Scan Image1");
            println!("first_item: {}", ent.path().display());

            let img_dyn = image::open(ent.path()).unwrap();
            let img = img_dyn.as_rgba8().unwrap();

            //for first image, just copy image onto the canvas
            canvas.copy_from(img, 0, 0);

            //compress/save image
            canvas.save(save_path).unwrap();
        }
        else //any subsequent image
        {
            println!("Scan Image2");
            println!("{}", ent.path().display());

            //for all other images, subtract image, then copy over image
            let img_dyn = image::open(ent.path()).unwrap();
            let img = img_dyn.as_rgba8().unwrap();

            //subtract the image
            for (x, y, pixel) in img.enumerate_pixels()
            {
                let mut canvas_pixel = canvas.get_pixel_mut(x,y);

                //TODO: disable debug mode to use alpha value
                let new_pixel = [
                    pixel[0] - canvas_pixel[0],
                    pixel[1] - canvas_pixel[1],
                    pixel[2] - canvas_pixel[2],
                    if debug_mode {255} else {pixel[3] - canvas_pixel[3]},
                ];

                *canvas_pixel = image::Rgba(new_pixel);
            }

            canvas.save(save_path).unwrap();

            //try compressing image
            {
                let canvas_as_raw = canvas.into_raw();

                let brotli_start = PreciseTime::now();
                compressor.write(&canvas_as_raw).unwrap();
                let brotli_end = PreciseTime::now();
                println!("Brotli compression took {} seconds", brotli_start.to(brotli_end));
            }

            //clear canvas (there must be a better way to do this?
            canvas = RgbaImage::new(canvas_width, canvas_height);

            //copy the just subtracted image onto canvas
            canvas.copy_from(img, 0, 0);
        }

        count += 1;
    }

}
