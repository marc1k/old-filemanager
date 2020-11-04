use {
    super::{
        Position,
        Buffer,
        Cell,
        Region
    },
    std::io::Write,
    crossterm::{
        terminal::{
            enable_raw_mode, disable_raw_mode,
            EnterAlternateScreen, LeaveAlternateScreen,
            Clear, ClearType
        },
        cursor::{
            self,
            MoveTo,
            MoveToNextLine
        },
        style::Print,
        QueueableCommand,
        Result,
    }
};

pub struct Viewport<W> where W: Write {
    buffer: Buffer,
    pub writer: W,
}

impl<W: Write> Viewport<W> {
    pub fn new(region: Region, writer: W) -> Self {
        Self {
            buffer: Buffer::filled(region, Cell::new('x')),
            writer
        }
    }

    pub fn draw(&mut self) {
        let w = &mut self.writer;

        let mut last: Option<Position> = None;

        for (pos, cell) in self.buffer.iter() {
            if let Some(last) = last {
                if !(pos.col == last.col + 1 && pos.row == last.row) {
                    w.queue(MoveTo(pos.col, pos.row)).unwrap();
                }
            }

            last = Some(pos.clone());

            w.queue(Print(cell)).unwrap();
        }

        w.flush().unwrap();
    }

    pub fn init_buffer(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.writer.queue(EnterAlternateScreen)?;
        self.writer.queue(cursor::Hide)?;
        self.writer.queue(cursor::DisableBlinking)?;
        self.writer.queue(MoveTo(0, 0))?;

        self.writer.flush()?;

        Ok(())
    }

    pub fn release_buffer(&mut self) -> Result<()> {
        self.writer.queue(cursor::EnableBlinking)?;
        self.writer.queue(cursor::Show)?;
        self.writer.queue(LeaveAlternateScreen)?;

        self.writer.flush()?;

        disable_raw_mode()
    }
}
