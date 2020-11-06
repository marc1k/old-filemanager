use criterion::{black_box, criterion_group, criterion_main, Criterion};
use viewport::*;

use crossterm::terminal;
use std::io::{Write, BufWriter, stderr};
use crossterm::{
    terminal::{
        enable_raw_mode, disable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen, Clear, ClearType,
    },
    cursor::{self, MoveTo, MoveToNextLine},
    style::Print,
    QueueableCommand,
};


pub fn draw(c: &mut Criterion) {
    c.bench_function("full draw", |b| {
        let stderr = stderr();
        let writer = BufWriter::new(stderr);
        let size: Size = terminal::size().map(Into::into).unwrap();
        let mut port = Viewport::new(writer, size);

        port.init_buffer();
        b.iter(|| {
            black_box(port.draw().unwrap())
        });
        port.release_buffer();

    });
}

criterion_group!(benches, draw);
criterion_main!(benches);

