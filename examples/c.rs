extern crate glium;
extern crate gltile;
extern crate yoga;
extern crate yoga_gltile;

use glium::DisplayBuild;
use yoga::{Backend, Builds, Renderable};

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1536, 1024)
        .build_glium()
        .unwrap();

    let mut renderer = gltile::Renderer::new(&display, 16, "examples/assets/tileset.png");

    let builder = yoga_gltile::Builder::new();

    let mut text = builder.view();
    text.set_height(3.0);
    text.set_align_self(yoga::Align::Center);
    text.set_flex_grow(1.0);
    text.set_background_color(
        Some(yoga::style::BackgroundColor::Color(*gltile::colors::YOGA_GRAY)));

    let mut image = builder.view();
    image.set_width(8.0);
    image.set_margin(yoga::Edge::End, 2.0);
    image.set_background_color(
        Some(yoga::style::BackgroundColor::Color(*gltile::colors::YOGA_TEAL)));

    let mut root = builder.view();
    root.set_width(50.0);
    root.set_height(12.0);
    root.set_flex_direction(yoga::FlexDirection::Row);
    root.set_padding(yoga::Edge::All, 2.0);
    root.set_background_color(Some(yoga::style::BackgroundColor::Color(*gltile::colors::WHITE)),);

    root.insert_child(&image, 0);
    root.insert_child(&text, 1);

    root.calculate_layout();

    let mut console = gltile::Console::new(renderer.size);
    let mut be = yoga_gltile::Backend::new();

    loop {
        console = be.render(&root, console);
        renderer
            .vertex_buffer
            .blit_console(&console, gltile::units::ScreenTile2D::new(0, 0));
        renderer.render();

        for ev in renderer.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
