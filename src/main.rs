use clap::Parser;
use reqwest::blocking::get;
use srgb;
use std::{thread::sleep, time::Duration};
use x11cap::*;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("stripe.local"))]
    endpoint: String,

    #[arg(short, long, default_value_t = 500, help="wait time before updating RGB values in ms")]
    wait_delay: u64,

    #[arg(short, long, default_value_t = false)]
    use_linear_rgb: bool,

    #[arg(short, long, default_value_t = String::from("http"))]
    protocol: String,

    #[arg(short, long, default_value_t = 0)]
    monitor: u8,
}

#[toml_cfg::toml_config]
struct Settings {
    #[default("")]
    stripe_server_endpoint: &'static str,
}

fn main() {
    let args = Args::parse();

    let mut endpoint: String = args.endpoint;

    // use compiled-in endpoint setting if it set
    if SETTINGS.stripe_server_endpoint.len() > 0 {
        endpoint = SETTINGS.stripe_server_endpoint.to_string();
    }

    println!(
        "Starting RGB LED Blacklight to endpoint: {}://{}...",
        args.protocol, endpoint
    );
    let mut capturer = Capturer::new(CaptureSource::Monitor(args.monitor.into())).unwrap();

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

        if args.use_linear_rgb {
            let [r, g, b] = srgb::gamma::linear_from_u8([tot_r, tot_g, tot_b]);

            tot_r = (r * 255.0) as u8;
            tot_g = (g * 255.0) as u8;
            tot_b = (b * 255.0) as u8;
        }

        let request_str = format!(
            "{}://{}/setRGBA?r={}&g={}&b={}",
            args.protocol, endpoint, tot_r, tot_g, tot_b
        );
        let resp = get(request_str);
        if resp.is_err() {
            eprintln!("{:?}", resp.err());
        }

        sleep(Duration::from_millis(args.wait_delay));
    }
}
