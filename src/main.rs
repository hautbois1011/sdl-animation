extern crate sdl2;

use sdl2::{
    rect::Rect,
    pixels::Color,
    pixels::PixelFormatEnum,
    event::Event,
    keyboard::Keycode
};

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

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
        .map_err(|e| e.to_string())?;
    texture.with_lock(None, |buf, pitch| {
        for y in 0..256 {
            for x in 0..256 {
                let offset = y*pitch + x*3;
                buf[offset] = x as u8;
                buf[offset+1] = y as u8;
                buf[offset+2] = 0;
            }
        }
    })?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
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

        frame += 1.0f64;
        canvas.copy_ex(&texture, None,
                       Some(Rect::new(200, 200, 256, 256)),
                       frame, None, false, false)?;

        canvas.present();
    }

    Ok(())
}
