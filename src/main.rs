extern crate sdl2;

use sdl2::{
    rect::Rect,
    pixels::Color,
    pixels::PixelFormatEnum,
    event::Event,
    keyboard::Keycode
};

use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 animation", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::IYUV, 256, 256)
        .map_err(|e| e.to_string())?;
    texture.with_lock(None, |buf, pitch| {
        let w = 256;
        let h = 256;

        for y in 0..h {
            for x in 0..w {
                let offset = y*pitch + x;
                buf[offset] = 128;
            }
        }

        let y_size = pitch*h;
        for y in 0..h/2 {
            for x in 0..w/2 {
                let u_offset = y_size + y*pitch/2 + x;
                let v_offset = y_size + (pitch/2 * h/2) + y*pitch/2 + x;
                buf[u_offset] = (x*2) as u8;
                buf[v_offset] = (y*2) as u8;
            }
        }
    })?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut frame = 0.0f64;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.clear();

        if frame >= 360.0f64 { frame = 0.0f64; }
        frame += 1.0f64;
        canvas.copy_ex(&texture, None,
                       Some(Rect::new(200, 200, 256, 256)),
                       frame, None, false, false)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}
