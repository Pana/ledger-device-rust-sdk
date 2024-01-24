extern "C" {
    fn screen_clear();
    fn screen_update();
    fn screen_set_keepout(x: u32, y: u32, width: u32, height: u32);
    fn bagl_hal_draw_rect(color: u32, x: i32, y: i32, width: u32, height: u32);
}

pub fn sdk_screen_clear() {
    unsafe {
        screen_clear();
    }
}

pub fn sdk_set_keepout(x: u32, y: u32, width: u32, height: u32) {
    unsafe {
        screen_set_keepout(x, y, width, height);
    }
}

pub fn sdk_screen_update() {
    unsafe {
        screen_update();
    }
}


pub fn sdk_bagl_hal_draw_rect(color: u32, x: i32, y: i32, width: u32, height: u32) {
    unsafe {
        bagl_hal_draw_rect(color, x, y, width, height);
    }
}
