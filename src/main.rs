extern crate sdl2;
extern crate time;
/*
extern crate piston;
extern crate piston_window;
extern crate sprite;
extern crate find_folder;


//use std::rc::Rc;

use piston_window::*;
use sprite::*;
use mode::*;

mod game;
mod mode;
mod event;
*/

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::path::Path;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("realm", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let png_house = Path::new("house.png");
    let png_grass = Path::new("grass.png");

    let mut renderer = window.renderer().present_vsync().build().unwrap();
    let texture_house = renderer.load_texture(png_house).unwrap();
    let texture_grass = renderer.load_texture(png_grass).unwrap();

    println!("{:?}", texture_house.query());

    let mut time_prev = time::precise_time_s();
    'mainloop: loop {
        let time_now = time::precise_time_s();
        let time_delta = time_now - time_prev;
        let fps = 1.00 / time_delta;
        time_prev = time_now;

        // println!("FPS: {}", fps);

        renderer.clear();
        for x in 0..10 {
            for y in 0..10 {
                renderer.copy(
                    &texture_grass,
                    None,
                    Some(Rect::new(x * 120, y * 120, 128, 128))
                ).expect("Render failed");
            }
        }

        for x in 0..10 {
            renderer.copy(
                &texture_house,
                None,
                Some(Rect::new(
                    x * 200 + 50,
                    100,
                    texture_house.query().width,
                    texture_house.query().height
                ))
            ).expect("Render failed");
        }

        renderer.present();

        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }

    /*
    let (width, height) = (960, 540);
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("realm", (width, height))
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

    let menu_mode = menu::Menu { title: "Main Menu".to_string() };
    let game = game::Game { mode: &menu_mode };

    let mut events = window.events();

    while let Some(e) = events.next(&mut window) {
        game.dispatch(&e);
        /*
        if let Some(r) = e.idle_args() {
            continue;
        }

        if let Some(r) = e.after_render_args() {
            continue;
        }

        if let Some(r) = e.render_args() {
            menu_mode.render(&r);
            continue;
        }

        if let Some(u) = e.update_args() {
            menu_mode.update(&u);
            continue;
        }

        if let Some(u) = e.press_args() {
            //menu_mode.input(&u);
        }
        */
    }

    /*

    for e in window.events() {
        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
        }

        mode.run(e);

        e.draw_2d(|c, g| {
            clear([0.2, 0.2, 0.0, 1.0], g);
        });
        if let Some(_) = e.press_args() {
            //std::process::exit(0);
        }
    }
    */
    */
}

