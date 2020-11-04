use {
    crate::{
        Result,
        tui::{
            Position,
            Dimensions,
            Buffer,
            Cell,
            Region,
            Viewport,
        }
    },
    super::Context,
    std::io::Write,
};

pub struct App {
    context: Context
}

impl App {
    /// Constructs an `App` within the specified context.
    pub fn with_context(context: Context) -> Self {
        Self {
            context
        }
    }

    pub async fn run(&self) -> Result<()> {
        let err = std::io::stderr();
        let writer = std::io::BufWriter::new(err.lock());

        let dim = Dimensions::from(crossterm::terminal::size().unwrap());

        let mut viewport = Viewport::new(Region::new(dim.width, dim.height), writer);

        viewport.init_buffer().unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        for _ in 0..10 {
            viewport.draw();
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        viewport.release_buffer().unwrap();
        
        Ok(())
    }
}
