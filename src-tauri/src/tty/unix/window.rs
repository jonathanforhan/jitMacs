use nix::libc::winsize;

#[allow(non_snake_case)]
#[derive(Clone, serde::Deserialize)]
pub struct WindowSize {
    numRows: u16,
    numCols: u16,
    cellWidth: u16,
    cellHeight: u16,
}

impl WindowSize {
    pub fn to_winsize(&self) -> winsize {
        winsize {
            ws_row: self.numRows,
            ws_col: self.numCols,
            ws_xpixel: self.cellWidth,
            ws_ypixel: self.cellHeight
        }
    }
}