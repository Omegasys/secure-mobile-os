pub struct Layer {
    pub id: u64,
    pub visible: bool,
    pub z_index: i32,
}

pub struct Compositor {
    layers: [Option<Layer>; 128],
}

impl Compositor {
    pub const fn new() -> Self {
        Self {
            layers: [None; 128],
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        for slot in self.layers.iter_mut() {
            if slot.is_none() {
                *slot = Some(layer);
                break;
            }
        }
    }

    pub fn render(&self) {
        // Sort by z-index and composite frames
    }
}
