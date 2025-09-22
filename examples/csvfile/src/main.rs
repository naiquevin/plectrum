use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;

use plectrum::{Enum, Plectrum};

#[derive(Debug, Plectrum)]
#[plectrum(rename_all = "snake_case")]
enum Status {
    Sold,
    Reserved,
    OutOfStock,
}

pub struct StatusModel(PathBuf);

impl plectrum::DataSource for StatusModel {
    type Id = u8;

    async fn load(&self) -> Result<HashMap<u8, String>, plectrum::Error> {
        let file = File::open(&self.0)
            .map_err(|e| plectrum::Error::DataSource(Box::new(e)))?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut m = HashMap::new();
        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here..
            let record = result.map_err(|e| plectrum::Error::DataSource(Box::new(e)))?;
            let id = record.get(0)
                .unwrap()
                .parse::<u8>()
                .map_err(|e| plectrum::Error::DataSource(Box::new(e)))?;
            let name = record.get(1).unwrap();
            m.insert(id, name.to_owned());
        }
        Ok(m)
    }
}

#[allow(deprecated)]
#[tokio::main]
async fn main() {
    let filepath = std::env::var("PLECTRUM_CSVFILE").unwrap_or(String::from("statuses.csv"));
    let model = StatusModel(PathBuf::from(filepath));
    match plectrum::Mapping::<u8, Status>::load(&model).await {
        Ok(mapping) => {
            dbg!(mapping.by_id(1));
            dbg!(mapping.by_value("out_of_stock"));

            // Deprecated but still works
            dbg!(mapping.get_id(&Status::Reserved));
            dbg!(Status::Reserved.id(&mapping));
        }
        Err(e) => {
            panic!("Failed to initialize mapping: {e:?}");
        }
    }
}
