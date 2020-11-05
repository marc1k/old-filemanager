use opal_lib::viewport::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bounds = Bounds::of(Size(10, 10)).at_origin(Position(2, 2));
    let buf = Buffer::filled_with(Cell::new("x".into()), bounds);

    for p in buf.bounds.iter() {
        println!("{:?}", p);
    }

    println!("\n{:?}", buf);

    Ok(())
}
