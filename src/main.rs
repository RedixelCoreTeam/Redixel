fn main() {
    if let Err(e) = red_pixel::init() {
        eprintln!("RedPixel Engine initialization error: {e}");
        std::process::exit(1);
    }
}
