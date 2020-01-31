extern crate sdl2;
extern crate glm;

use sdl2::{
    rect::Rect,
    rect::Point,
    pixels::Color,
    pixels::PixelFormatEnum,
    event::Event,
    keyboard::Keycode
};

use glm::ext;
use glm::*;

use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 animation", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
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

    let projection = ext::perspective(ext::consts::quarter_pi(), 4.0 / 3.0, 0.1, 100.0);
    let view = ext::look_at(
        Vector3::new(4., 3., 3.),
        Vector3::new(0., 0., 0.),
        Vector3::new(0., 1., 0.)
    );
    let mut model = Matrix4::new(
        Vector4::new(1., 0., 0., 0.),
        Vector4::new(0., 1., 0., 0.),
        Vector4::new(0., 0., 1., 0.),
        Vector4::new(0., 0., 0., 1.)
    );

    let points = vec![
        Vector4::new(0., 0., 0., 1.),
        Vector4::new(1., 0., 0., 1.),
        Vector4::new(1., 1., 0., 1.),
        Vector4::new(0., 1., 0., 1.),
        Vector4::new(0., 0., 1., 1.),
        Vector4::new(1., 0., 1., 1.),
        Vector4::new(1., 1., 1., 1.),
        Vector4::new(0., 1., 1., 1.)
    ];

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

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        if frame >= 360.0f64 { frame = 0.0f64; }
        frame += 1.0f64;
        canvas.copy_ex(&texture, None,
                       Some(Rect::new(400, 400, 64, 64)),
                       frame, None, false, false)?;

        model = ext::rotate(&model, 0.01, Vector3::new(0.3, 0.4, 0.5));
        let mvp = projection * view * model;
        let points_render = points.iter().map(|&x| {
            let y = mvp * x;
            Point::new((200. * y.x/y.z) as i32, (200. * y.y/y.z) as i32) + Point::new(200, 300)
        }).collect::<Vec<_>>();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_line(points_render[0], points_render[1])?;
        canvas.draw_line(points_render[1], points_render[2])?;
        canvas.draw_line(points_render[2], points_render[3])?;
        canvas.draw_line(points_render[3], points_render[0])?;
        canvas.draw_line(points_render[4], points_render[5])?;
        canvas.draw_line(points_render[5], points_render[6])?;
        canvas.draw_line(points_render[6], points_render[7])?;
        canvas.draw_line(points_render[7], points_render[4])?;
        canvas.draw_line(points_render[0], points_render[4])?;
        canvas.draw_line(points_render[1], points_render[5])?;
        canvas.draw_line(points_render[2], points_render[6])?;
        canvas.draw_line(points_render[3], points_render[7])?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}
