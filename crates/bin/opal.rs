use {viewport::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Buffer::blank(Bounds::of(4, 4));
    let bound = Bounds::of(4, 1).at_origin(0, 0);

    for x in buf.iter_bounds(&bound) {
        println!("{:?}", x);
    }

    Ok(())
}
