pub trait Renderable {
    fn render(&self) -> String;
}

//Body
pub struct Body {
    items: Vec<Box<dyn Renderable>>,
}

impl Body {
    pub fn new() -> Self {
        let _vec = Vec::<Box<dyn Renderable>>::new();
        Body { items: _vec }
    }
}
impl Renderable for Body {
    fn render(&self) -> String {
        let mut master = String::new();
        let start_tag = "<body>";
        let end_tag = "</body>";

        master.push_str(&start_tag);
        for child in &self.items {
            let text = child.render();
            master.push_str(text.as_str());
        }
        master.push_str(&end_tag);
        master
    }
}

pub struct Text {
    pub text: String,
    pub style: u8,
}

impl Text {
    /*
     * Add styling to text and return it as a String
     */
    fn get_text(&self) -> String {
        let mut s = self.text.clone();
        if self.style & crate::parser::parser::ITALIC_STYLE != 0 {
            s = format!("<i>{}</i>", s);
        }
        if self.style & crate::parser::parser::BOLD_STYLE != 0 {
            s = format!("<b>{}</b>", s);
        }
        if self.style & crate::parser::parser::CODE_STYLE != 0 {
            s = self.text.clone();
            s = format!("<code>{}</code>", s);
        }

        return s;
    }
}

pub struct CodeBlock {
    pub texts: Vec<Text>,
}
impl CodeBlock {
    pub fn new() -> Self {
        CodeBlock { texts: vec![] }
    }
}
impl Renderable for CodeBlock {
    fn render(&self) -> String {
        let mut master = String::new();
        let start_tag = "<pre><code>";
        let end_tag = "</code></pre>";

        master.push_str(&start_tag);
        // Iterate over all the styled text and generate a paragraph
        for text in &self.texts {
            let s: String = text.get_text();
            master.push_str(&s);
        }
        master.push_str(&end_tag);
        master
    }
}

pub struct Paragraph {
    pub texts: Vec<Text>,
}

impl Paragraph {
    pub fn new() -> Self {
        Paragraph { texts: vec![] }
    }
}
impl Renderable for Paragraph {
    fn render(&self) -> String {
        let mut master = String::new();
        let start_tag = "<p>";
        let end_tag = "</p>";

        master.push_str(&start_tag);
        // Iterate over all the styled text and generate a paragraph
        for text in &self.texts {
            let s: String = text.get_text();
            master.push_str(&s);
        }
        master.push_str(&end_tag);
        master
    }
}
//Head
pub struct Head {
    text: String,
}

impl Head {
    pub fn new() -> Self {
        Head {
            text: String::new(),
        }
    }
}
impl Renderable for Head {
    fn render(&self) -> String {
        let mut master = String::new();
        let start_tag = "<head>";
        let end_tag = "</head>";

        master.push_str(&start_tag);
        master.push_str(&self.text);
        master.push_str(&end_tag);
        master
    }
}
// Heading
pub struct Heading {
    text: Paragraph,
    level: u8,
}

impl Heading {
    pub fn new(text: Paragraph, level: u8) -> Self {
        Heading {
            text: text,
            level: level,
        }
    }
}
impl Renderable for Heading {
    fn render(&self) -> String {
        let mut master = String::new();
        let start_tag = format!("<h{}>", self.level);
        let end_tag = format!("</h{}>", self.level);

        master.push_str(&start_tag);
        master.push_str(&self.text.render());
        master.push_str(&end_tag);
        master
    }
}

pub struct HTML {
    pub items: Vec<Box<dyn Renderable>>,
}

impl HTML {
    pub fn new() -> Self {
        let _vec = Vec::<Box<dyn Renderable>>::new();
        HTML { items: _vec }
    }
}
impl Renderable for HTML {
    fn render(&self) -> String {
        let mut master = String::new();
        let start_tag = "<html>\n";
        let end_tag = "</html>";

        master.push_str(&start_tag);
        for child in &self.items {
            let text = child.render();
            master.push_str(text.as_str());
            master.push_str("\n");
        }
        master.push_str(&end_tag);
        master
    }
}
