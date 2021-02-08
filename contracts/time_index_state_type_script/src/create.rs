use crate::error::*;
use crate::helper::*;
use ckb_std::ckb_constants::Source;

pub fn create(script_hash: [u8; 32]) -> Result<(), Error> {
    //should only one time index cell in output
    if get_script_hash_cell_count(script_hash, Source::Output) != 1 {
        return Err(Error::InvalidTimeIndexOutput);
    }

    //the args of output script should equal the output point of the first input
    cell_args_check_when_create()?;

    let output_cell_data = crate::helper::load_cell_data(script_hash, Source::Output)?;
    if output_cell_data.len() != TIME_INDEX_CELL_DATA_LEN as usize
        || output_cell_data[0] >= TIME_INDEX_CELL_DATA_N
        || output_cell_data[1] != TIME_INDEX_CELL_DATA_N
    {
        return Err(Error::InvalidCellData);
    }

    //index should equal 0 when create
    if output_cell_data[0] != 0 {
        return Err(Error::InvalidCellData);
    }
    Ok(())
}
