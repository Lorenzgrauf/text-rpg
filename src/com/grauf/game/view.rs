pub(crate) trait View {
    fn next_view(&self) -> Box<dyn View>;
}

pub(crate) struct Menu<'a> {
    pub(crate) view_factory: &'a Box<dyn FnOnce() -> Box<dyn View>>,
}

impl Menu<'a> {
    pub(crate) fn new<'a, F>(view_factory: &'a F) -> Menu where F: FnOnce() -> Box<dyn View> {
        Menu {
            view_factory:
        }
    }
}

impl View for Menu {

    fn next_view(&self) -> Box<dyn View> {
        (&self.view_factory)()
    }
}