mod core;
mod space;

pub use crate::core::{
    Buffer,
    Cell
};

pub use space::{
    Position,
    Region,
    Size
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_contains() {
        let reg = Region::new(Size::of(5, 5));

        assert!(reg.contains(Position::of(0, 0)) == true);
        assert!(reg.contains(Position::of(10, 0)) == false);
        assert!(reg.contains(Position::of(5, 5)) == false);
        assert!(reg.contains(Position::of(3, 2)) == true);
    }

    #[test]
    fn region_contains_iter() {
        let reg = Region::new(Size::of(5, 5));

        for pos in reg.iter_relative() {
            assert!(reg.contains(pos));
        }
    }

    #[test]
    fn buf_split() {
        use std::io::{
            stderr,
            BufWriter,
            Write
        };

        use crossterm::{
            style::Print,
            terminal::{
                disable_raw_mode,
                enable_raw_mode,
                Clear,
                ClearType,
                EnterAlternateScreen,
                LeaveAlternateScreen
            },
            QueueableCommand
        };

        let stderr = stderr();
        let mut writer = BufWriter::new(stderr.lock());

        let region = Region::new(crossterm::terminal::size().unwrap().into());
        let (t, b) = region.vsplit(10);
        let buf = Buffer::fill(Cell::new(".".to_string()), t);
        let buf2 = Buffer::fill(Cell::new("o".to_string()), b);

        enable_raw_mode().unwrap();
        writer.queue(EnterAlternateScreen).unwrap();
        writer.queue(crossterm::cursor::Hide).unwrap();
        writer.flush().unwrap();

        let mut last_pos: Option<Position> = None;
        for (pos, cell) in buf.iter_absolute() {
            if let Some(last) = last_pos {
                if last.x + 1 != pos.x || last.y != pos.y {
                    writer
                        .queue(crossterm::cursor::MoveTo(pos.x, pos.y))
                        .unwrap();
                }
            }

            last_pos = Some(pos);

            writer.queue(Print(cell)).unwrap();
        }
        for (pos, cell) in buf2.iter_absolute() {
            if let Some(last) = last_pos {
                if last.x + 1 != pos.x || last.y != pos.y {
                    writer
                        .queue(crossterm::cursor::MoveTo(pos.x, pos.y))
                        .unwrap();
                }
            }

            last_pos = Some(pos);

            writer.queue(Print(cell)).unwrap();
        }
        writer.flush().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(3000));

        writer.queue(crossterm::cursor::Show).unwrap();
        writer.queue(LeaveAlternateScreen).unwrap();
        writer.flush().unwrap();
        disable_raw_mode().unwrap();
    }
}
