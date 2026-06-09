use std::collections::BTreeMap;

#[derive(Clone)]
pub enum WidgetType {
    Clock,
    Calendar,
    Weather,
    Battery,
    Notes,
    SecurityMonitor,
    NetworkMonitor,
    Custom,
}

#[derive(Clone)]
pub struct Widget {
    pub id: u64,
    pub name: String,
    pub widget_type: WidgetType,

    pub width: u8,
    pub height: u8,

    pub enabled: bool,
}

pub struct WidgetManager {
    widgets: BTreeMap<u64, Widget>,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: BTreeMap::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Widget) {
        self.widgets.insert(widget.id, widget);
    }

    pub fn remove_widget(&mut self, id: u64) {
        self.widgets.remove(&id);
    }

    pub fn enable_widget(&mut self, id: u64) {
        if let Some(widget) = self.widgets.get_mut(&id) {
            widget.enabled = true;
        }
    }

    pub fn disable_widget(&mut self, id: u64) {
        if let Some(widget) = self.widgets.get_mut(&id) {
            widget.enabled = false;
        }
    }

    pub fn get_widget(&self, id: u64) -> Option<&Widget> {
        self.widgets.get(&id)
    }

    pub fn widget_count(&self) -> usize {
        self.widgets.len()
    }

    pub fn enabled_widgets(&self) -> Vec<&Widget> {
        self.widgets
            .values()
            .filter(|widget| widget.enabled)
            .collect()
    }

    pub fn render_all(&self) {
        for widget in self.widgets.values() {
            if widget.enabled {
                println!(
                    "Rendering widget: {}",
                    widget.name
                );
            }
        }
    }
}

impl Default for WidgetManager {
    fn default() -> Self {
        Self::new()
    }
}
