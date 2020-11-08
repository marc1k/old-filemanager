use opal_lib::viewport::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let universe = Region::new((10, 10).into());
    let mut viewport_initial = Buffer::fill(Cell::new(".".to_string()), universe);
    let mut viewport = Buffer::fill(Cell::new(".".to_string()), universe);

    println!("intial");
    println!("{:?}", viewport);

    let mut reg = Region::new((3, 2).into());
    reg.set_origin((2, 2).into());
    let component = Buffer::fill(Cell::new("x".to_string()), reg);

    viewport += component.clone();

    println!("add 3x2 block");
    println!("{:?}", viewport);

    let diff = viewport - viewport_initial;


    println!("diff");
    println!("{:?}", diff);

    Ok(())
}
