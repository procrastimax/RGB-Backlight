use reqwest::blocking::get;
use srgb;
use std::{thread::sleep, time::Duration};
use x11cap::*;

#[toml_cfg::toml_config]
struct Settings {
    #[default("stripe.local")]
    stripe_server_endpoint: &'static str,
    #[default(false)]
    use_linear_rgb: bool,
}

fn main() {
    println!(
        "Starting RGB LED Blacklight to endpoint: {}...",
        SETTINGS.stripe_server_endpoint
    );
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

        let mut tot_r: u8 = (tot_r / size) as u8;
        let mut tot_g: u8 = (tot_g / size) as u8;
        let mut tot_b: u8 = (tot_b / size) as u8;

        if SETTINGS.use_linear_rgb {
            let [r, g, b] = srgb::gamma::linear_from_u8([tot_r, tot_g, tot_b]);

            tot_r = (r * 255.0) as u8;
            tot_g = (g * 255.0) as u8;
            tot_b = (b * 255.0) as u8;
        }

        let request_str = format!(
            "https://{}/setRGBA?r={}&g={}&b={}",
            SETTINGS.stripe_server_endpoint, tot_r, tot_g, tot_b
        );
        let resp = get(request_str);
        if resp.is_err() {
            eprintln!("{:?}", resp.err());
        }

        sleep(Duration::from_millis(200));
    }
}
