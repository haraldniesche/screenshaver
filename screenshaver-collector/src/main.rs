
use screenshaver_collector::get_screenshot;
use std::time::{Instant};

fn main() {
    println!("Taking Screenshot:");
    let then = Instant::now();
    let s = get_screenshot(0).unwrap();
    let now = Instant::now();
	println!(
        "{width} x {height} x {depth} = {len} bytes in {duration}ms", 
        width = s.width(), 
        height = s.height(), 
        depth = s.pixel_width(), 
        len = s.raw_len(),
        duration = (now - then).as_millis()
    );

    let theEnd = Instant::now();
}
