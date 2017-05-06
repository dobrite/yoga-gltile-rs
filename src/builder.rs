use gltile;
use measurer;
use yoga;

pub struct Builder {
    measurer: measurer::Measurer,
}

impl Builder {
    pub fn new() -> Self {
        Builder { measurer: measurer::Measurer {} }
    }
}

impl<'meas> yoga::Builds<'meas, gltile::colors::Rgb> for Builder {
    fn create_context<'text>(&'meas self, text: &'text str) -> Box<yoga::Context<'text, 'meas>> {
        Box::new(yoga::Context::new(text, &self.measurer))
    }

    fn view<'r>(&self) -> yoga::View<'r, gltile::colors::Rgb> {
        yoga::View::new()
    }

    fn text<'text>(&'meas self, text: &'text str) -> yoga::Text<'text, 'meas, gltile::colors::Rgb> {
        yoga::Text::new(text, self.create_context(text))
    }
}
