use crate::create::*;
use crate::error::Error;
use alloc::vec::Vec;
use ckb_std::error::SysError;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    high_level::*,
};

pub const TIME_INDEX_CELL_DATA_LEN: u8 = 2;
pub const TIME_INDEX_CELL_DATA_N: u8 = 12;

pub fn cell_data_check(cell_data: &Vec<u8>) -> Result<(), Error> {
    if cell_data.len() != TIME_INDEX_CELL_DATA_LEN as usize
        || cell_data[0] >= TIME_INDEX_CELL_DATA_N
        || cell_data[1] != TIME_INDEX_CELL_DATA_N
    {
        return Err(Error::InvalidCellData);
    }
    Ok(())
}

fn get_script_hash_cell_count(script_hash: [u8; 32], source: Source) -> usize {
    QueryIter::new(load_cell_type_hash, source)
        .filter(|type_hash| match type_hash {
            Some(type_script_hash) => *type_script_hash == script_hash,
            None => false,
        })
        .count()
}

pub fn cell_input_check(script_hash: [u8; 32]) -> Result<(), Error> {
    //should only one time index cell in input
    if get_script_hash_cell_count(script_hash, Source::Input) != 1 {
        return Err(Error::InvalidTimeIndexInput);
    }
    Ok(())
}

pub fn cell_output_check(script_hash: [u8; 32]) -> Result<(), Error> {
    //should only one time index cell in output
    if get_script_hash_cell_count(script_hash, Source::Output) != 1 {
        return Err(Error::InvalidTimeIndexOutput);
    }
    Ok(())
}

pub fn create_cell_args_check() -> Result<(), Error> {
    let time_index_script = load_script()?;
    let input_out_point = load_input_out_point(0, Source::Input)?;
    if input_out_point.as_bytes()[..] != time_index_script.args().raw_data()[..] {
        return Err(Error::InvalidArgument);
    }
    Ok(())
}

pub fn get_position_of_cell_with_type_script(
    script_hash: [u8; 32],
    source: Source,
) -> Option<usize> {
    QueryIter::new(load_cell_type_hash, source).position(|type_script_op| match type_script_op {
        Some(type_script) => type_script == script_hash,
        None => false,
    })
}

pub fn update_cell_args_check(script_hash: [u8; 32]) -> Result<(), Error> {
    let script = load_script()?;
    let script_args: Bytes = script.args().unpack();
    if script_args.is_empty() {
        return Err(Error::InvalidArgument);
    }

    let cell_index = match get_position_of_cell_with_type_script(script_hash, Source::Input) {
        Some(position) => position,
        None => return Err(Error::InvalidTimeIndexInput),
    };
    let input_cell_data = load_cell(cell_index, Source::Input)?;
    let input_script = match input_cell_data.type_().to_opt() {
        Some(type_script) => type_script,
        None => Err(Error::InvalidTimeIndexInput),
    };
    let input_script_args: Bytes = input_script.args().unpack();

    if input_script_args[..] != script_args[..] {
        return Err(Error::InvalidArgument);
    }
    Ok(())
}

pub fn load_cell_data(script_hash: [u8; 32], source: Source) -> Result<Vec<u8>, Error> {
    let cell_index = match get_position_of_cell_with_type_script(script_hash, source) {
        Some(position) => position,
        None => match source {
            Source::Input | Source::GroupInput => Err(Error::InvalidTimeIndexInput),
            Source::Output | Source::GroupOutput => Err(Error::InvalidTimeIndexOutput),
            _ => Err(Error::ItemMissing),
        },
    };
    match ckb_std::high_level::load_cell_data(cell_index, source) {
        Some(cell_data) => cell_data,
        Err(sys_err) => Err(Error::from(sys_err)),
    }
}

pub fn create_time_index_check(time_index: u8) -> Result<(), Error> {
    //index should equal 0 when create
    if time_index != 0 {
        return Err(Error::InvalidCellData);
    }
    Ok(())
}

pub fn update_time_index_check(input_time_index: u8, output_time_index: u8) -> Result<(), Error> {
    if input_time_index != TIME_INDEX_CELL_DATA_N - 1 && output_time_index != input_time_index + 1
        || input_time_index == TIME_INDEX_CELL_DATA_N - 1 && output_time_index != 0
    {
        return Err(Error::InvalidCellData);
    }
    Ok(())
}
