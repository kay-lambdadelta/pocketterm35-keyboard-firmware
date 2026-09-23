use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};
use rp2040_hal::gpio::{DynPinId, FunctionSioInput, FunctionSioOutput, Pin, PullDown};

use crate::keymap::{COLUMNS, ROWS};

pub type RowPin = Pin<DynPinId, FunctionSioInput, PullDown>;
pub type ColumnPin = Pin<DynPinId, FunctionSioOutput, PullDown>;

pub struct KeyMatrix {
    rows: [RowPin; ROWS],
    cols: [ColumnPin; COLUMNS],
    current: [[bool; COLUMNS]; ROWS],
    previous: [[bool; COLUMNS]; ROWS],
}

impl KeyMatrix {
    pub fn new(rows: [RowPin; ROWS], columns: [ColumnPin; COLUMNS]) -> Self {
        Self {
            rows,
            cols: columns,
            current: [[false; COLUMNS]; ROWS],
            previous: [[false; COLUMNS]; ROWS],
        }
    }

    pub fn scan(&mut self, delay: &mut impl DelayNs) {
        self.previous = self.current;

        for (column_index, column_pin) in self.cols.iter_mut().enumerate() {
            column_pin.set_high().unwrap();

            delay.delay_us(20);

            for (row_index, row_pin) in self.rows.iter_mut().enumerate() {
                self.current[row_index][column_index] = row_pin.is_high().unwrap();
            }

            column_pin.set_low().unwrap();
        }
    }

    #[inline]
    pub fn is_pressed(&self, row: usize, column: usize) -> bool {
        self.current[row][column]
    }

    #[inline]
    pub fn is_edge(&self, row: usize, column: usize) -> bool {
        self.current[row][column] && !self.previous[row][column]
    }

    #[inline]
    pub fn row(&self, row: usize) -> &[bool; COLUMNS] {
        &self.current[row]
    }
}
