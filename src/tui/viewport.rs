use {
    super::{
        W,
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

pub struct Viewport {
    buffer: Buffer,
    pub writer: W,
}

impl Viewport {
    pub fn new(region: Region, writer: W) -> Self {
        Self {
            buffer: Buffer::filled(region, Cell::new('x')),
            writer
        }
    }

    pub fn draw(&mut self) {
        for (pos, cell) in self.buffer.iter() {
            self.writer.queue(MoveTo(pos.col, pos.row)).unwrap();
            self.writer.queue(Print(cell)).unwrap();
        }

        self.writer.flush().unwrap();
    }

    pub fn init(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.writer.queue(EnterAlternateScreen)?;
        self.writer.queue(cursor::Hide)?;
        self.writer.queue(cursor::DisableBlinking)?;
        self.writer.queue(MoveTo(0, 0))?;

        self.writer.flush()?;

        Ok(())
    }

    pub fn release(&mut self) -> Result<()> {
        self.writer.queue(cursor::EnableBlinking)?;
        self.writer.queue(cursor::Show)?;
        self.writer.queue(LeaveAlternateScreen)?;

        self.writer.flush()?;

        disable_raw_mode()
    }
}
