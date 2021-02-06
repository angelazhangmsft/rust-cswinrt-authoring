use bindings::authoring_demo::*;
use bindings::*;
use futures::executor::block_on;

fn main() -> windows::Result<()> {
    let example = Example::new()?;
    example.set_sample_property(42)?;
    println!("{}", example.sample_property()?);
    let hello = Example::say_hello()?.to_string();
    println!("{}", hello);

    let folder_enumerator = FolderEnumeration::new()?;
    let future = folder_enumerator.get_files_and_folders_async()?;
    block_on(future)?;
    let files = folder_enumerator.all_files()?.to_string();
    println!("{}", files);

    Ok(())
}
