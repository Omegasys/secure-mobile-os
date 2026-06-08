pub struct Plugin {
    pub id: u64,
    pub name: &'static str,
    pub active: bool,
}

pub struct PluginLoader {
    plugins: [Option<Plugin>; 64],
}

impl PluginLoader {
    pub const fn new() -> Self {
        Self {
            plugins: [None; 64],
        }
    }

    pub fn load_plugin(&mut self, plugin: Plugin) {
        for slot in self.plugins.iter_mut() {
            if slot.is_none() {
                *slot = Some(plugin);
                break;
            }
        }
    }

    pub fn activate(&mut self, id: u64) {
        for plugin in self.plugins.iter_mut().flatten() {
            if plugin.id == id {
                plugin.active = true;
            }
        }
    }

    pub fn deactivate(&mut self, id: u64) {
        for plugin in self.plugins.iter_mut().flatten() {
            if plugin.id == id {
                plugin.active = false;
            }
        }
    }
}
