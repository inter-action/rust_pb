extern crate rustpb;

use std::io::stdout;
use std::{thread, time};

use rustpb::Bar;


fn main() {
    let mut bar = Bar::new(100.0);
    let mut w = stdout();
    let mut prg = 0.0;
    bar.text("loading");

    while prg <= 100.0 {
        bar.value(prg);
        let _ = bar.write_to(&mut w);
        thread::sleep(time::Duration::from_millis(30));
        prg += 1.0;
    }
}
