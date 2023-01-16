use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
struct RGB(u8, u8, u8);

trait Format {
    fn to_hex(&self) -> String;
    fn to_hex_short(&self) -> String;
    fn to_rgb(&self) -> String;
    fn to_rgba(&self, a: f32) -> String;
    fn randomize(&mut self);
}

impl Format for RGB {
    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    fn to_hex_short(&self) -> String {
        format!("#{:X}{:X}{:X}", self.0, self.1, self.2)
    }

    fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.0, self.1, self.2)
    }

    fn to_rgba(&self, a: f32) -> String {
        format!("rgba({}, {}, {}, {})", self.0, self.1, self.2, a)
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.0 = rng.gen_range(0..255);
        self.1 = rng.gen_range(0..255);
        self.2 = rng.gen_range(0..255);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let help_flag = args.iter().position(|x| x == "--help");

    if help_flag.is_some() || args.len() < 2 {
        println!("Usage: {} <number>", args[0]);
        println!("Usage: {} <number> --a <float>", args[0]);
        println!("Usage: {} <number> --f <file_name>", args[0]);
        println!("Usage: {} <number> --j", args[0]);
        println!(
            "Usage: {} <number> --a <float> --f <file_name> --j",
            args[0]
        );
        return;
    }

    let number = args[1].parse::<u8>().unwrap();

    let alpha_flag = args.iter().position(|x| x == "--a");

    let alpha = match alpha_flag {
        Some(_) => args[alpha_flag.unwrap() + 1].parse::<f32>().unwrap(),
        None => 1.0,
    };

    let write_to_file = args.iter().position(|x| x == "--f");

    let write_to_json = args.iter().position(|x| x == "--j");

    let mut file = match write_to_file {
        Some(_) => {
            let file_name = args[write_to_file.unwrap() + 1].clone();
            Some(File::create(file_name).unwrap())
        }
        None => None,
    };

    let mut colors: Vec<RGB> = vec![];

    for _ in 0..number {
        let mut color = RGB(0, 0, 0);
        color.randomize();
        colors.push(color);
    }

    for color in &colors {
        println!("Hex: {}", color.to_hex());
        println!("Hex short: {}", color.to_hex_short());
        println!("RGB: {}", color.to_rgb());
        println!("RGBA: {}", color.to_rgba(alpha));
        println!();
    }

    if write_to_file.is_some() {
        let fwf = match file {
            Some(ref mut file) => file,
            None => return,
        };

        for color in &colors {
            fwf.write_all(b"Hex: ").unwrap();
            fwf.write_all(color.to_hex().as_bytes()).unwrap();
            fwf.write_all(b"\nHex short: ").unwrap();
            fwf.write_all(color.to_hex_short().as_bytes()).unwrap();
            fwf.write_all(b"\nRGB: ").unwrap();
            fwf.write_all(color.to_rgb().as_bytes()).unwrap();
            fwf.write_all(b"\nRGBA: ").unwrap();
            fwf.write_all(color.to_rgba(alpha).as_bytes()).unwrap();

            fwf.write_all(b"\n\n").unwrap();
        }

        fwf.sync_all().unwrap();
    }

    if write_to_json.is_some() {
        let mut file = File::create("colors.json").unwrap();

        let mut json = String::from("[");

        for color in &colors {
            json.push_str(&format!(
                "{{\"hex\":\"{}\",\"hex_short\":\"{}\",\"rgb\":\"{}\",\"rgba\":\"{}\"}},",
                color.to_hex(),
                color.to_hex_short(),
                color.to_rgb(),
                color.to_rgba(alpha)
            ));
        }

        json.pop();

        json.push_str("]");

        file.write_all(json.as_bytes()).unwrap();

        file.sync_all().unwrap();
    }
}
