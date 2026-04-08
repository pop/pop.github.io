#[derive(Clone, PartialEq)]
pub struct WindowState {
    pub game_id: String,
    pub open: bool,
    pub pos: (i32, i32),
    pub z_index: u32,
}

#[derive(Clone, PartialEq)]
pub struct WindowManager {
    pub windows: Vec<WindowState>,
    pub z_counter: u32,
}

impl WindowManager {
    pub fn new(game_ids: Vec<String>) -> Self {
        let windows = game_ids
            .into_iter()
            .enumerate()
            .map(|(i, id)| WindowState {
                game_id: id,
                open: false,
                pos: (50 + (i as i32 * 30), 50 + (i as i32 * 30)),
                z_index: 100,
            })
            .collect();
        WindowManager {
            windows,
            z_counter: 100,
        }
    }
}
