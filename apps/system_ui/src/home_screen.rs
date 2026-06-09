use std::collections::BTreeMap;

#[derive(Clone)]
pub struct AppIcon {
    pub app_id: u64,
    pub title: String,
    pub package_name: String,
    pub row: u8,
    pub column: u8,
}

#[derive(Clone)]
pub struct Widget {
    pub id: u64,
    pub name: String,
    pub width: u8,
    pub height: u8,
}

pub struct HomePage {
    pub page_number: usize,
    pub icons: Vec<AppIcon>,
    pub widgets: Vec<Widget>,
}

pub struct HomeScreen {
    pages: BTreeMap<usize, HomePage>,
    current_page: usize,
}

impl HomeScreen {
    pub fn new() -> Self {
        let mut pages = BTreeMap::new();

        pages.insert(
            0,
            HomePage {
                page_number: 0,
                icons: Vec::new(),
                widgets: Vec::new(),
            },
        );

        Self {
            pages,
            current_page: 0,
        }
    }

    pub fn add_page(&mut self) {
        let index = self.pages.len();

        self.pages.insert(
            index,
            HomePage {
                page_number: index,
                icons: Vec::new(),
                widgets: Vec::new(),
            },
        );
    }

    pub fn switch_page(&mut self, page: usize) -> Result<(), &'static str> {
        if self.pages.contains_key(&page) {
            self.current_page = page;
            Ok(())
        } else {
            Err("page does not exist")
        }
    }

    pub fn add_icon(
        &mut self,
        page: usize,
        icon: AppIcon,
    ) -> Result<(), &'static str> {
        match self.pages.get_mut(&page) {
            Some(home_page) => {
                home_page.icons.push(icon);
                Ok(())
            }
            None => Err("page not found"),
        }
    }

    pub fn add_widget(
        &mut self,
        page: usize,
        widget: Widget,
    ) -> Result<(), &'static str> {
        match self.pages.get_mut(&page) {
            Some(home_page) => {
                home_page.widgets.push(widget);
                Ok(())
            }
            None => Err("page not found"),
        }
    }

    pub fn remove_icon(
        &mut self,
        page: usize,
        package_name: &str,
    ) -> Result<(), &'static str> {
        let page = self.pages.get_mut(&page)
            .ok_or("page not found")?;

        page.icons.retain(|icon| {
            icon.package_name != package_name
        });

        Ok(())
    }

    pub fn current_page(&self) -> Option<&HomePage> {
        self.pages.get(&self.current_page)
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn current_page_index(&self) -> usize {
        self.current_page
    }
}

impl Default for HomeScreen {
    fn default() -> Self {
        Self::new()
    }
}
