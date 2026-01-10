use crate::game::{collision::get_ground_height, player};
use core::fmt::Write;

use crate::utils::console::Console;

pub fn display_pos() {
    if let Some(player) = player::as_mut() {
        let pos = player.position();
        let (x, y, z) = (pos.x, pos.y, pos.z);
        let angle = player.rotation().y;
        let speed = player.obj_base.speed;
        // let height = get_ground_height(&mut player.acch);
        let mut console = Console::with_pos_and_size(0f32, 120f32, 120f32, 85f32);
        console.set_bg_color(0x0000007F);
        console.set_font_color(0xFFFFFFFF);
        console.set_font_size(0.25f32);
        console.set_dynamic_size(true);
        let _ = console.write_fmt(format_args!("pos:\nx:{x:>9.2}\ny:{y:>9.2}\nz:{z:>9.2}\n"));
        // let _ = console.write_fmt(format_args!("grd height: {height:.2}\n"));
        let _ = console.write_fmt(format_args!("angle: {angle}\n"));
        let _ = console.write_fmt(format_args!("speed: {speed:.2}"));
        console.draw(true);
    }
}
