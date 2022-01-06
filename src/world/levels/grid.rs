use crate::prelude::*;

const TOTAL_ROWS: usize = 18;
const TOTAL_COLUMNS: usize = 28;

const LEVEL_TEST: [&str; TOTAL_ROWS] = [
    "",
    "","","","",
    "",
    "","","",
    "              X             ",
    "","","",
    "",
    "","","",
    "",
];

const LEVEL_1: [&str; TOTAL_ROWS] = [
    "XXXXX     XXXXXXXX     XXXXX",
    "","","","",
    "   XXXXXXX        XXXXXXX   ",
    "","","",
    "   XXXXXXXXXXXXXXXXXXXXXX   ",
    "","","",
    "XXXXXXXXX          XXXXXXXXX",
    "","","",
    "XXXXX     XXXXXXXX     XXXXX",
];

const LEVEL_2: [&str; TOTAL_ROWS] = [
    "XXXX    XXXXXXXXXXXX    XXXX",
    "","","","",
    "    XXXXXXXXXXXXXXXXXXXX    ",
    "","","",
    "XXXXXX                XXXXXX",
    "      X              X      ",
    "       X            X       ",
    "        X          X        ",
    "         X        X         ",
    "","","",
    "XXXX    XXXXXXXXXXXX    XXXX",
];

const LEVEL_3: [&str; TOTAL_ROWS] = [
    "XXXX    XXXX    XXXX    XXXX",
    "","","","",
    "  XXXXXXXX        XXXXXXXX  ",
    "","","",
    "XXXX      XXXXXXXX      XXXX",
    "","","",
    "    XXXXXX        XXXXXX    ",
    "","","",
    "XXXX    XXXX    XXXX    XXXX"
];

const LEVELS : [[&str; TOTAL_ROWS]; 4] = [LEVEL_TEST, LEVEL_1, LEVEL_2, LEVEL_3];

pub fn load_level(level: LevelNumber) -> LevelGrid {
    LevelGrid::from(level)
}

pub struct LevelGrid {
    inner: [&'static str; TOTAL_ROWS]
}

impl From<LevelNumber> for LevelGrid {
    fn from(from: LevelNumber) -> Self {
        Self { inner: LEVELS[*from] }
    }
}

impl LevelGrid {
    pub fn iter(&self) -> LevelGridIterator {
        LevelGridIterator {
            column: 0,
            row: 0,
            grid: self
        }
    }
}

pub struct LevelGridIterator<'a> {
    column: usize,
    row: usize,
    grid: &'a LevelGrid
}

impl<'a> LevelGridIterator<'a> {
    fn navigate_to_populated_row(&mut self) -> Option<()> {
        if self.row == TOTAL_ROWS {
            return None;
        }
        
        if self.grid.inner[self.row].len() == 0 {
            self.row += 1;
            return self.navigate_to_populated_row()
        }

        Some(())
    }

    fn navigate_to_populated_column(&mut self) -> Option<()> {
        if self.column == TOTAL_COLUMNS {
            return None;
        }

        if let Some(block) = self.grid.inner[self.row].chars().nth(self.column) {
            if block == ' ' {
                self.column += 1;
                return self.navigate_to_populated_column();
            }

            return Some(())
        }

        None
    }
}

impl<'a> Iterator for LevelGridIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.navigate_to_populated_row().is_none() {
            return None;
        }

        if self.navigate_to_populated_column().is_none() {
            self.row += 1;
            self.column = 0;
            return self.next();
        }
        
        let position = Some((self.row, self.column));
        self.column += 1;

        position
    }
}