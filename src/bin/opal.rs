use {
    opal::{
        App,
        RawArgs,
        Context
    },
    std::convert::TryFrom,
    clap::Clap
};

#[tokio::main]
async fn main() -> opal::Result<()> {
    let context = Context::try_from(RawArgs::parse())?;
    let app = App::with_context(context);

    if let Err(e) = app.run().await {
        eprintln!("error: {}", e);
    }

    Ok(())
}
