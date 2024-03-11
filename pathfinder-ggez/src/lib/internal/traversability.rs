use crate::lib::grid::Grid;
use crate::lib::Index;

pub fn is_visible(idx1: &Index, idx2: &Index, grid: &Grid) -> bool {
    let mut dx = idx2.idx_lon.abs_diff(idx1.idx_lon) as i32;
    let mut dy = idx2.idx_lat.abs_diff(idx1.idx_lat) as i32;
    let mut x_lon = idx1.idx_lon as i32;
    let mut y_lat = idx1.idx_lat as i32;
    let mut n = 1 + dx + dy;
    let x_inc: i32;
    if idx2.idx_lon > idx1.idx_lon {
        x_inc = 1;
    } else {
        x_inc = -1;
    }
    let y_inc: i32;
    if idx2.idx_lat > idx1.idx_lat {
        y_inc = 1;
    } else {
        y_inc = -1;
    }
    let mut error: i32 = dx - dy;
    dx = dx * 2;
    dy = dy * 2;

    while n > 0 {
        if grid.is_traversable(&Index::new(y_lat as u32, x_lon as u32)) == 0 {
            return false;
        }
        if error > 0 {
            x_lon += x_inc;
            error -= dy;
        } else {
            y_lat += y_inc;
            error += dx;
        }
        n -= 1;
    }

    true
}
