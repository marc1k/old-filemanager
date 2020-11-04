use opal_lib::{viewport::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf =
        Buffer::filled_with(Cell::new(".".into()), Bounds::of(8, 8));

    let bound = Bounds::of(3, 3).at_origin(1, 1);
    buf.iter_intersect_mut(&bound)
        .for_each(|(_, c)| c.content = "0".into());

    print!("{}", buf);

    Ok(())
}
