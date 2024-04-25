use std::fmt::{Debug, Formatter};
use itertools::Itertools;
use crate::{Cell, Item, Row};

impl Debug for Row{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.cells.iter().map(|cell|format!("{:?}-{:?}",cell.text,cell.dimensions)).join(",");
        writeln!(f, "{}", s)?;
        Ok(())
    }

}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{:?}",self.text)?;
        Ok(())
    }
}

impl Debug for Item {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}",self.id)?;
        Ok(())
    }

}
