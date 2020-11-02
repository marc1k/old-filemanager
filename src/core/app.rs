use {
    crate::{
        Result,
        tui::{
            Position,
            Dimension,
            Buffer,
            Cell,
            Region,
            Viewport,
        }
    },
    super::{
        Context
    },
    std::io::Write,
    crossterm::{
        terminal::{
            enable_raw_mode, disable_raw_mode,
            EnterAlternateScreen, LeaveAlternateScreen,
            Clear, ClearType
        },
        QueueableCommand,
    },
};

pub struct App {
    context: Context
}

impl App {
    /// Constructs an `App` within the specified context
    pub fn with_context(context: Context) -> Self {
        Self {
            context
        }
    }

    pub async fn run(&self) -> Result<()> {
        let dim = Dimension::from(crossterm::terminal::size().unwrap());
        //let dim = Dimension::from((80, 24));
        let writer = std::io::BufWriter::new(std::io::stderr());

        let mut viewport = Viewport::new(Region::new(dim.width, dim.height), writer);

        viewport.init().unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        viewport.draw();

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        viewport.release().unwrap();
        
        Ok(())
    }
}
