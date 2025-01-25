use colored::Colorize;
use orientation::Axis;
use strum::EnumCount;

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub start: [usize; Axis::COUNT],
    pub end: [usize; Axis::COUNT],
}

impl BoundingBox {
    pub fn size(&self) -> [usize; Axis::COUNT] {
        let [start_x, start_y, start_z] = self.start;
        let [end_x, end_y, end_z] = self.end;

        [
            end_x - start_x + 1,
            end_y - start_y + 1,
            end_z - start_z + 1,
        ]
    }
}

impl ToString for BoundingBox {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for y in 0..super::MAX_SIZE {
            result.push_str("\n");
            result.push_str(&format!("Bounding box layer: {}", y));
            for z in (0..super::MAX_SIZE).rev() {
                result.push_str("\n");
                for x in 0..super::MAX_SIZE {
                    if self.start == [x, y, z] {
                        result.push_str(&"  ".on_blue().to_string());
                    } else if self.end == [x, y, z] {
                        result.push_str(&"  ".on_green().to_string());
                    } else {
                        result.push_str(&"  ".on_truecolor(0, 0, 0).to_string())
                    }
                }
            }
        }

        result
    }
}
