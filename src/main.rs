use rand::Rng;

#[derive(Clone, Copy)]
struct RGB(u8, u8, u8);

impl RGB {
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

    if args.len() < 2 {
        println!("Usage: {} <number>", args[0]);
        return;
    }

    let number = args[1].parse::<u8>().unwrap();

    let mut colors: Vec<RGB> = vec![];

    for _ in 0..number {
        let mut color = RGB(0, 0, 0);
        color.randomize();
        colors.push(color);
    }

    for color in colors {
        println!("Hex: {}", color.to_hex());
        println!("Hex short: {}", color.to_hex_short());
        println!("RGB: {}", color.to_rgb());
        println!("RGBA: {}", color.to_rgba(0.5));
        println!();
    }
}
