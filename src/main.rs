mod render;
mod traits;
mod utils;

fn main() {
    let mut hm = utils::height_map::init_height_map::<25, 25>(-1_f32);
    utils::height_map::create_land(&mut hm, 100);
    utils::height_map::create_lakes(&mut hm, 15);

    let hm = utils::height_map::smooth_height_map(hm);

    let _sea = utils::height_map::init_height_map::<25, 25>(-1_f32);
    // println!("{hm:?}");

    // let shm = utils::height_map::smooth_height_map(hm);
    // println!("{shm:?}");

    // utils::height_map::create_land(&mut hm, 150);
    // println!("{hm:?}");

    // utils::height_map::create_lakes(&mut hm, 20);
    // utils::height_map::print_height_map(&hm);

    let (mut glium_render, event_loop) = render::glium::GliumRender::new("Teste");

    let colors_height_map = ([1.0, 1.0, 1.0], [0.6, 0.0, 0.0], [0.2, 0.0, 0.0]);
    glium_render.add_mesh(render::glium::util::height_map_to_mesh(
        hm,
        colors_height_map,
        &glium_render.display,
    ));

    let colors_sea = ([1_f32; 3], [0_f32, 0_f32, 0.6_f32], [0_f32, 0_f32, 0.2_f32]);
    glium_render.add_mesh(render::glium::util::height_map_to_mesh(
        utils::height_map::init_height_map::<25, 25>(-1_f32),
        colors_sea,
        &glium_render.display,
    ));

    event_loop.run(move |event, _, context| {
        if *context == glium::glutin::event_loop::ControlFlow::Exit {
            return;
        }

        match event {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *context = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glium::glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == glium::glutin::event::ElementState::Pressed {
                        let key = match input.virtual_keycode {
                            Some(key) => key,
                            None => return,
                        };

                        let vec_pos = match key {
                            glium::glutin::event::VirtualKeyCode::W => [0_f32, 0_f32, 1_f32],
                            glium::glutin::event::VirtualKeyCode::S => [0_f32, 0_f32, -1_f32],
                            glium::glutin::event::VirtualKeyCode::A => [-1_f32, 0_f32, 0_f32],
                            glium::glutin::event::VirtualKeyCode::D => [1_f32, 0_f32, 0_f32],
                            glium::glutin::event::VirtualKeyCode::Space => [0_f32, 1_f32, 0_f32],
                            glium::glutin::event::VirtualKeyCode::LShift => [0_f32, -1_f32, 0_f32],
                            _ => [0_f32; 3],
                        };

                        glium_render.add_camera(vec_pos);
                    }
                }
                _ => (),
            },
            _ => (),
        }

        glium_render.draw_scene();
    })
}
