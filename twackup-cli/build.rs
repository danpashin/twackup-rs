use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    EmitBuilder::builder()
        .all_build()
        .all_git()
        .all_cargo()
        .fail_on_error()
        .emit()?;
    Ok(())
}
