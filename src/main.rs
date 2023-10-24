use std::time::Duration;

use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::env;
use std::thread;

const WIDTH: usize = 180;
const HEIGHT: usize = 64;

fn main() {
    let _args: String = env::args().collect::<String>();
    let animation: bool = _args
        .split(' ')
        .into_iter()
        .find(|s| *s == "animation")
        .is_some();

    let mut rng = SmallRng::seed_from_u64(0);

    let steps = 250;

    // let mut playground: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    // let mut playground_buffer: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];

    let mut playground: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
    let mut playground_buffer: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

    for _ in 0..WIDTH * HEIGHT / 10 {
        let (y, x) = (rng.gen_range(0..HEIGHT), rng.gen_range(0..WIDTH));
        playground[y][x] = true;
    }

    if animation {
        let calculation_start = chrono::Utc::now();

        for _ in 0..steps {
            for xi in 0..WIDTH {
                for yi in 0..HEIGHT {
                    let total = 0
                        + playground[(yi - 1) % HEIGHT][(xi - 1) % WIDTH] as u8
                        + playground[(yi - 1) % HEIGHT][xi] as u8
                        + playground[(yi - 1) % HEIGHT][(xi + 1) % WIDTH] as u8
                        + playground[yi][(xi - 1) % WIDTH] as u8
                        + playground[yi][(xi + 1) % WIDTH] as u8
                        + playground[(yi + 1) % HEIGHT][(xi - 1) % WIDTH] as u8
                        + playground[(yi + 1) % HEIGHT][xi] as u8
                        + playground[(yi + 1) % HEIGHT][(xi + 1) % WIDTH] as u8;
                    playground_buffer[yi][xi] = conways_rule(playground[yi][xi], total);
                }
            }
            playground = playground_buffer;
            print_playground(&playground, true);
            thread::sleep(Duration::from_millis(50));
        }
        let duration = chrono::Utc::now() - calculation_start;

        println!(
            // "calculation took [ {} ] nanoseconds for {} steps on a {} x {} grid.",
            "calculation took [ {} ] milliseconds for {} steps on a {} x {} grid.",
            // duration.num_nanoseconds().unwrap(),
            duration.num_milliseconds(),
            steps,
            WIDTH,
            HEIGHT
        );
    } else {
        print_playground(&playground, false);

        for _ in 0..steps {
            for xi in 0..WIDTH {
                for yi in 0..HEIGHT {
                    let total = 0
                        + playground[(yi - 1) % HEIGHT][(xi - 1) % WIDTH] as u8
                        + playground[(yi - 1) % HEIGHT][xi] as u8
                        + playground[(yi - 1) % HEIGHT][(xi + 1) % WIDTH] as u8
                        + playground[yi][(xi - 1) % WIDTH] as u8
                        + playground[yi][(xi + 1) % WIDTH] as u8
                        + playground[(yi + 1) % HEIGHT][(xi - 1) % WIDTH] as u8
                        + playground[(yi + 1) % HEIGHT][xi] as u8
                        + playground[(yi + 1) % HEIGHT][(xi + 1) % WIDTH] as u8;
                    playground_buffer[yi][xi] = conways_rule(playground[yi][xi], total);
                }
            }
            playground = playground_buffer;
            print_playground(&playground, true);
            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn conways_rule(cell_state: bool, total_neigbors: u8) -> bool {
    match (cell_state, total_neigbors) {
        (true, 0 | 1) => false,
        (true, 2 | 3) => true,
        (false, 3) => true,
        (true, 4..) => false,
        (false, _) => false,
    }
}

#[allow(dead_code)]
fn print_playground(playground: &[[bool; WIDTH]; HEIGHT], overwrite: bool) {
    if overwrite {
        print!("\r");
        for _ in 0..HEIGHT {}
    }
    // print!("{esc}[1;1H", esc = 27 as char);

    print!("{esc}c", esc = 27 as char);

    for double_row in playground.chunks(2) {
        for (top, bottom) in double_row[0].into_iter().zip(double_row[1]) {
            match (top, bottom) {
                (true, true) => print!("█"),
                (true, false) => print!("▀"),
                (false, true) => print!("▄"),
                (false, false) => print!(" "),
            }
        }
        println!();
    }
}
