use yoga;
use yoga::Measures;

pub struct Measurer {}

impl Measures for Measurer {
    fn measure(&self, text: &str) -> yoga::Size {
        yoga::Size {
            width: text.len() as f32,
            height: 1.0,
        }
    }
}
