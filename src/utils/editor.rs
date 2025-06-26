use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;

pub fn edit(file_path: &str, target_row: usize, target_col: usize, new_value: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(file_path)?;
    let mut records: Vec<csv::StringRecord> = Vec::new();
    
    for result in reader.records() {
        records.push(result?);
    }
    
        if target_row > 0 && target_row <= records.len() {
        let record = &mut records[target_row - 1];
        if target_col > 0 && target_col <= record.len() {
            let mut new_record = csv::StringRecord::new();
            for (i, field) in record.iter().enumerate() {
                if i == target_col - 1 {
                    new_record.push_field(new_value);
                } else {
                    new_record.push_field(field);
                }
            }
            records[target_row - 1] = new_record;
        }
    }
    
    let mut writer = WriterBuilder::new().from_path(file_path)?;
    for record in records {
        writer.write_record(&record)?;
    }
    writer.flush()?;
    
    Ok(())
}