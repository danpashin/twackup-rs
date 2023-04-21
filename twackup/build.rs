use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    EmitBuilder::builder().all_git().fail_on_error().emit()?;
    Ok(())
}
