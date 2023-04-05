use clap::Parser;
use srgb;
use std::net::UdpSocket;
use std::{thread::sleep, time::Duration};
use x11cap::*;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("stripe.local"))]
    endpoint: String,

    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "delay before updating RGB values in ms"
    )]
    delay: u64,

    #[arg(short, long, default_value_t = false)]
    use_linear_rgb: bool,

    #[arg(short, long, default_value_t = 0)]
    monitor: u8,

    #[arg(
        short,
        long,
        default_value_t = 3,
        help = "color change threshold as a sum of all channels to be exceeded in order to trigger an RGB value change"
    )]
    threshold: u64,

    #[arg(short, long, default_value_t = 15)]
    window_length: i32,
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

    println!("Starting RGB LED Blacklight to endpoint: {}...", endpoint);
    let mut capturer = Capturer::new(CaptureSource::Monitor(args.monitor.into())).unwrap();
    let geo = capturer.get_geometry();
    let size = geo.width as u64 * geo.height as u64;

    let (mut last_r, mut last_g, mut last_b) = (0, 0, 0);

    let udp_stream = UdpSocket::bind("0.0.0.0:8777").unwrap();
    udp_stream.connect(format!("{}:80", endpoint)).unwrap();
    println!("conected...");

    let mut ema_r: u8 = 0;
    let mut ema_g: u8 = 0;
    let mut ema_b: u8 = 0;
    let alpha: f32 = 2.0 / (args.window_length as f32 + 1.0);

    loop {
        let ps = capturer.capture_frame().unwrap();

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

        ema_r = (alpha * tot_r as f32 + (1.0 - alpha) * ema_r as f32) as u8;
        ema_g = (alpha * tot_g as f32 + (1.0 - alpha) * ema_g as f32) as u8;
        ema_b = (alpha * tot_b as f32 + (1.0 - alpha) * ema_b as f32) as u8;

        let curr_sum: u64 = ema_r as u64 + ema_g as u64 + ema_b as u64;
        let last_sum: u64 = last_r as u64 + last_g as u64 + last_b as u64;

        if curr_sum.abs_diff(last_sum) > args.threshold {
            let package = format!("r={},g={},b={}\n", ema_r, ema_g, ema_b);
            udp_stream.send(package.as_bytes()).unwrap();
        }
        (last_r, last_g, last_b) = (ema_r, ema_g, ema_b);
        sleep(Duration::from_millis(args.delay));
    }
}
