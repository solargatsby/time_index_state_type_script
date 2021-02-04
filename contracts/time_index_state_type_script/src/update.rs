use ckb_std::{ckb_constants::Source, };
use crate::error::*;
use crate::helper::*;

pub fn update(script_hash:[u8; 32]) -> Result<(), Error>{
    cell_input_check(script_hash)?;
    cell_output_check(script_hash)?;
    update_cell_args_check(script_hash)?;

    let input_cell_data = crate::helper::load_cell_data(script_hash, Source::Input)?;
    cell_data_check(&input_cell_data)?;
    let output_cell_data = crate::helper::load_cell_data(script_hash, Source::Output)?;
    cell_data_check(&output_cell_data)?;

    let input_time_index = input_cell_data[0];
    let output_time_index = output_cell_data[0];
    update_time_index_check(input_time_index, output_time_index)?;
    Ok(())
}