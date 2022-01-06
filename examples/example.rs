use blackbody::{temperature_to_rgb, temperature_to_xyz};

fn main() {
    dbg!(temperature_to_rgb(3000.0));
    dbg!(temperature_to_xyz(3000.0));
}
