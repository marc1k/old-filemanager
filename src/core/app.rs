use {
    crate::Result,
    super::{
        Context
    }
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

        Ok(())
    }
}
