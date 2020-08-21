use anyhow::Result;
use serde::Serialize;

pub(crate) fn print_json<T: Serialize>(data: T) -> Result<()> {
    let serialized =
        serde_json::to_string_pretty(&data).map_err(selectel_mks::error::Error::SerializeError)?;

    println!("{}", serialized);

    Ok(())
}
