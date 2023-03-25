use std::fmt::format;
use std::thread::sleep;
use std::time::Duration;
use x11cap::*;

use reqwest::blocking::get;

fn main() {
    let mut capturer = Capturer::new(CaptureSource::Monitor(0)).unwrap();

    loop {
        let ps = capturer.capture_frame().unwrap();

        let geo = capturer.get_geometry();
        let size = geo.width as u64 * geo.height as u64;

        let (mut tot_r, mut tot_g, mut tot_b) = (0, 0, 0);

        for &Bgr8 { r, g, b, .. } in ps.as_slice().iter() {
            tot_r += r as u64;
            tot_g += g as u64;
            tot_b += b as u64;
        }

        let tot_r = tot_r / size;
        let tot_g = tot_g / size;
        let tot_b = tot_b / size;

        //println!("Avg: {:?}", (tot_r / size, tot_g / size, tot_b / size));

        let request_str = format!(
            "https://stripe.fritz.box/SetRGBA&r={}?g={}?b={}",
            tot_r, tot_g, tot_b
        );
        println!("making request: {}", request_str);

        let resp = get(request_str)?

        sleep(Duration::from_millis(1000));
    }
}
