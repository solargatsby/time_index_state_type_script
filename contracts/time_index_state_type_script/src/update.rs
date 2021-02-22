use ckb_std::ckb_constants::Source;

use crate::error::*;
use crate::helper::{
    check_args_when_update_cell, check_cell_data, get_script_hash_cell_count,
    TIME_INDEX_CELL_DATA_N,
};

pub fn update(script_hash: [u8; 32]) -> Result<(), Error> {
    //should only one time index cell in input
    if get_script_hash_cell_count(script_hash, Source::Input) != 1 {
        return Err(Error::InvalidTimeIndexInput);
    }
    //should only one time index cell in output
    if get_script_hash_cell_count(script_hash, Source::Output) != 1 {
        return Err(Error::InvalidTimeIndexOutput);
    }
    //check whether args of script of input not empty and equal args of output's
    check_args_when_update_cell(script_hash)?;

    let input_cell_data = crate::helper::load_cell_data(script_hash, Source::Input)?;
    check_cell_data(&input_cell_data)?;
    let output_cell_data = crate::helper::load_cell_data(script_hash, Source::Output)?;
    check_cell_data(&output_cell_data)?;

    let input_time_index = input_cell_data[0];
    let output_time_index = output_cell_data[0];
    if (input_time_index != TIME_INDEX_CELL_DATA_N - 1 && output_time_index != input_time_index + 1)
        || (input_time_index == TIME_INDEX_CELL_DATA_N - 1 && output_time_index != 0)
    {
        return Err(Error::InvalidCellData);
    }
    Ok(())
}
