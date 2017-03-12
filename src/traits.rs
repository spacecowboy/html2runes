use html5ever::rcdom::Handle;

pub trait HtmlConverter {
    fn convert_html(&mut self, handle: Handle) -> String;
}
