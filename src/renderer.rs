use Builder;
use gltile;
use pixset;
use yoga;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    fn walk(
        &mut self,
        node: &yoga::Renderable<gltile::colors::Rgb>,
        mut console: gltile::Console,
    ) -> gltile::Console {
        let width = node.get_layout_width() as i32;
        let height = node.get_layout_height() as i32;
        let top = node.get_layout_top() as i32;
        let left = node.get_layout_left() as i32;

        let color = match *node.get_color() {
            Some(c) => c,
            None => *gltile::colors::WHITE,
        };

        let background_color = match *node.get_background_color() {
            Some(yoga::style::BackgroundColor::Transparent) => *gltile::colors::BLACK,
            Some(yoga::style::BackgroundColor::Color(c)) => c,
            None => *gltile::colors::BLACK,
        };

        if let Some(text) = node.get_text() {
            for (pix, offset) in pixset::Str::from(text).iter() {
                console
                    .with_loc(gltile::units::ScreenTile2D::new(left + offset.0, top))
                    .with_fg(color)
                    .with_bg(background_color)
                    .set_pix(pix);
            }
        } else {
            for y in top..(top + height) {
                for x in left..(left + width) {
                    console
                        .with_loc(gltile::units::ScreenTile2D::new(x as i32, y as i32))
                        .with_fg(color)
                        .with_bg(background_color)
                        .set_pix(pixset::Pix::Empty);
                }
            }
        }

        for i in 0..node.get_child_count() {
            let child = node.get_child(i).unwrap();
            console = self.walk(child, console)
        }
        console
    }
}

impl<'meas> yoga::Renders<'meas> for Renderer {
    type Color = gltile::colors::Rgb;
    type Input = gltile::Console;
    type Output = Self::Input;
    type Builder = Builder;

    fn render(&mut self, node: &yoga::Renderable<Self::Color>, input: Self::Input) -> Self::Output {
        self.walk(node, input)
    }
}
