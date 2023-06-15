mod render;
mod scene;
mod traits;
mod utils;

const ANIMALS_MOVE_DELAY: u128 = 1500;
const ISLAND_WIDTH: usize = 120;
const ISLAND_HEIGHT: usize = 120;

const CAMERA_SENSI: f32 = 0.15_f32;

fn main() {
    let (mut glium_render, event_loop) = render::glium::GliumRender::new("Ilha", [15_f32; 3]);

    let mut scene = scene::Scene::<ISLAND_WIDTH, ISLAND_HEIGHT>::new(&glium_render.display);

    scene.create_entities(2, scene::EntityType::Animal1);
    scene.create_entities(2, scene::EntityType::Animal2);

    scene.create_entities(10, scene::EntityType::Plant1);
    scene.create_entities(10, scene::EntityType::Plant2);

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

                        let direction = match key {
                            glium::glutin::event::VirtualKeyCode::W => {
                                render::glium::camera::WalkDirection::Front
                            }
                            glium::glutin::event::VirtualKeyCode::S => {
                                render::glium::camera::WalkDirection::Back
                            }
                            glium::glutin::event::VirtualKeyCode::D => {
                                render::glium::camera::WalkDirection::Left
                            }
                            glium::glutin::event::VirtualKeyCode::A => {
                                render::glium::camera::WalkDirection::Right
                            }
                            glium::glutin::event::VirtualKeyCode::Space => {
                                render::glium::camera::WalkDirection::Up
                            }
                            glium::glutin::event::VirtualKeyCode::LShift => {
                                render::glium::camera::WalkDirection::Down
                            }
                            glium::glutin::event::VirtualKeyCode::Up => {
                                glium_render.rotate_camera((0_f32, -1.5_f32));
                                return;
                            }
                            glium::glutin::event::VirtualKeyCode::Down => {
                                glium_render.rotate_camera((0_f32, 1.5_f32));
                                return;
                            }
                            glium::glutin::event::VirtualKeyCode::Right => {
                                glium_render.rotate_camera((-1.5_f32, 0_f32));
                                return;
                            }
                            glium::glutin::event::VirtualKeyCode::Left => {
                                glium_render.rotate_camera((1.5_f32, 0_f32));
                                return;
                            }
                            glium::glutin::event::VirtualKeyCode::Minus => {
                                glium_render.zoom_out();
                                return;
                            }
                            glium::glutin::event::VirtualKeyCode::Equals => {
                                glium_render.zoom_in();
                                return;
                            }
                            _ => return,
                        };

                        glium_render.walk(direction);
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

                    // glium_render.rotate_camera((
                    //     CAMERA_SENSI * delta_x as f32,
                    //     CAMERA_SENSI * delta_y as f32,
                    // ));
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
