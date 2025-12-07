fn main() {
    if let Err(e) = red_pixel::init() {
        println!("Redixel Engine Initialization Failed {e}")
    }
}
