mod render;
mod scene;
mod traits;
mod utils;

const ANIMALS_MOVE_DELAY: u128 = 1500;
const ISLAND_WIDTH: usize = 120;
const ISLAND_HEIGHT: usize = 120;

fn main() {
    let (mut glium_render, event_loop) = render::glium::GliumRender::new("Ilha", [15_f32; 3]);

    let mut scene = scene::Scene::<ISLAND_WIDTH, ISLAND_HEIGHT>::new(&glium_render.display);

    scene.create_entities(5, scene::EntityType::Animal1);
    scene.create_entities(5, scene::EntityType::Animal2);

    scene.create_entities(45, scene::EntityType::Plant1);
    scene.create_entities(45, scene::EntityType::Plant2);

    glium_render.add_mesh(scene.get_height_map_mesh(&glium_render.display));

    let colors_sea = ([1_f32; 3], [0_f32, 0_f32, 0.6_f32], [0_f32, 0_f32, 0.2_f32]);
    glium_render.add_mesh(render::glium::util::height_map_to_mesh(
        utils::height_map::init_height_map::<ISLAND_WIDTH, ISLAND_HEIGHT>(-1_f32),
        colors_sea,
        &glium_render.display,
    ));

    let mut last_x = 0_f64;
    let mut last_y = 0_f64;
    let mut first_move = true;
    let mut time = std::time::SystemTime::now();

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
                            glium::glutin::event::VirtualKeyCode::W => [0_f32, -1_f32, 0_f32],
                            glium::glutin::event::VirtualKeyCode::S => [0_f32, 1_f32, 0_f32],
                            glium::glutin::event::VirtualKeyCode::D => [1_f32, 0_f32, 0_f32],
                            glium::glutin::event::VirtualKeyCode::A => [-1_f32, 0_f32, 0_f32],
                            glium::glutin::event::VirtualKeyCode::Space => [0_f32, 0_f32, 1_f32],
                            glium::glutin::event::VirtualKeyCode::LShift => [0_f32, 0_f32, -1_f32],
                            _ => [0_f32; 3],
                        };

                        glium_render.add_camera(vec_pos);
                    }
                }
                glium::glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    let delta_x = last_x - position.x;
                    let delta_y = last_y - position.y;

                    last_x = position.x;
                    last_y = position.y;
                    if first_move {
                        first_move = false;
                    }

                    glium_render.rotate_camera((1.5_f32, 1.5_f32));
                }
                _ => (),
            },
            _ => (),
        }

        if let Ok(delay) = std::time::SystemTime::now().duration_since(time) {
            if delay.as_millis() >= ANIMALS_MOVE_DELAY {
                scene.move_animals();
                time = std::time::SystemTime::now();
            }
        }

        glium_render.draw_scene(&mut scene);
    })
}
