use bindings::authoring_demo::*;
use bindings::authoring_demo2::*;
use bindings::*;

fn main() -> windows::Result<()> {
    let example = Example::new()?;
    //let example2 = Example2::new()?;

    example.set_sample_property(42)?;
    //example2.set_sample_property(50)?;

    //println!("{}", example.sample_property()?);
    // println!("{}", example2.sample_property()?);

    Ok(())
}
