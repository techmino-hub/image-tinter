# Image tinter
This module was made because for some reason some of the images in the website appears too bright, and there isn't an easy way to darken it like how love2d does with `love.graphics.setColor()`. I decided to just make an image processor to do it beforehand.  

## Running
This project uses Rust, so you need to install it first.  
Then, you can run it with `cargo run -r`.  

## Configuration
You can change some constants in `/src/main.rs` to modify the program's behavior:  
- `INPUT_DIR`: The directory where images are loaded from. It defaults to `./input/`.
- `OUTPUT_DIR`: The directory where the output images will be saved to. It defaults to `./output/`.
- `RED_MULTIPLIER`: The multiplier for the red channel. It defaults to `0.859`.
- `GREEN_MULTIPLIER`: The multiplier for the green channel. It defaults to `0.812`.
- `BLUE_MULTIPLIER`: The multiplier for the blue channel. It defaults to `0.808`.

The default RGB multipliers are the values that replicate Techmino's modeicon tint.