use ckb_std::ckb_constants::Source;

use crate::error::*;
use crate::helper::{check_args_when_create_cell, check_cell_data, get_script_hash_cell_count};

pub fn create(script_hash: [u8; 32]) -> Result<(), Error> {
    //should only one time index cell in output
    if get_script_hash_cell_count(script_hash, Source::Output) != 1 {
        return Err(Error::InvalidTimeIndexOutput);
    }

    //the args of output script should equal the output point of the first input
    check_args_when_create_cell()?;

    let output_cell_data = crate::helper::load_cell_data(script_hash, Source::Output)?;
    check_cell_data(&output_cell_data)?;

    //index should equal 0 when create
    if output_cell_data[0] != 0 {
        return Err(Error::InvalidCellData);
    }
    Ok(())
}
