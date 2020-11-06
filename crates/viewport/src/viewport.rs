use {
    crate::{Bounds, Buffer, Cell, Position, Size},
    std::io::Write,
    crossterm::{
        terminal::{
            enable_raw_mode, disable_raw_mode, EnterAlternateScreen,
            LeaveAlternateScreen, Clear, ClearType,
        },
        cursor::{self, MoveTo, MoveToNextLine},
        style::Print,
        QueueableCommand,
    },
};

/// The crate's interface to the terminal buffer. The structure whose state primarily consists
/// of an intermediate `Buffer` type holding information about `Cell`s at `Position`s.
pub struct Viewport<W> where W: Write {
    pub writer: W,
    
    /// An intermediate buffer representing all of the screen space.
    pub buf: Buffer,
}

impl<W: Write> Viewport<W> {
    pub fn new(writer: W, size: Size) -> Self {
        let bounds = Bounds::of(size).at_origin(Position(0, 0));
        let buf = Buffer::blank(bounds);

        Self {
            writer,
            buf,
        }
    }

    pub fn draw(
        &mut self,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let w = &mut self.writer;

        let mut prev_pos: Option<Position> = None;

        w.queue(MoveTo(self.buf.bounds.origin.0, self.buf.bounds.origin.1))?;

        for (pos, cell) in self.buf.iter() { 
            match prev_pos {
                // The case where the cursor is in the correct position.
                Some(ref prev) if prev.0 + 1 == pos.0 && prev.1 == pos.1 => {}

                // The case where the cursor is misplaced (row change)
                Some(_) => {
                    w.queue(MoveTo(pos.0, pos.1))?;
                }

                None => {}
            };

            prev_pos = Some(pos);

            w.queue(Print(cell))?;
        }

        w.flush()?;

        Ok(())
    }

    pub fn init_buffer(
        &mut self,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        self.writer.queue(EnterAlternateScreen)?;
        self.writer.queue(cursor::Hide)?;
        self.writer.queue(cursor::DisableBlinking)?;
        self.writer.queue(MoveTo(0, 0))?;

        self.writer.flush()?;

        Ok(())
    }

    pub fn release_buffer(
        &mut self,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        self.writer.queue(cursor::EnableBlinking)?;
        self.writer.queue(cursor::Show)?;
        self.writer.queue(LeaveAlternateScreen)?;

        self.writer.flush()?;

        disable_raw_mode()?;

        Ok(())
    }
}
