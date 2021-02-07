use crate::error::*;
use crate::helper::*;
use ckb_std::ckb_constants::Source;

pub fn create(script_hash: [u8; 32]) -> Result<(), Error> {
    cell_output_check(script_hash)?;
    create_cell_args_check()?;

    let output_cell_data = crate::helper::load_cell_data(script_hash, Source::Output)?;
    cell_data_check(&output_cell_data)?;

    create_time_index_check(output_cell_data[0])?;
    Ok(())
}
