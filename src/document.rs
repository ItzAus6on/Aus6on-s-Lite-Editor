use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open(filepath: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(filepath)?;
        let mut rows: Vec<Row> = Vec::new();
        for value in content.lines() {
            rows.push(Row::from(value));
        }
        Ok( Self {
            rows,
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }   

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}