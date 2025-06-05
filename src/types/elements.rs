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
        let start_tag =  "<body>";
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

//Paragraph
pub struct Paragraph {
    text: String,
}

impl Paragraph {
    pub fn new() -> Self {
        Paragraph { text: String::new() } 
    }
}
impl Renderable for Paragraph {
    fn render(&self) -> String {
        let mut master = String::new();
        let start_tag =  "<p>";
        let end_tag = "</p>";

        master.push_str(&start_tag);
        master.push_str(&self.text);
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
        Head { text: String::new() } 
    }
}
impl Renderable for Head {
    fn render(&self) -> String {
        let mut master = String::new();
        let start_tag =  "<head>";
        let end_tag = "</head>";

        master.push_str(&start_tag);
        master.push_str(&self.text);
        master.push_str(&end_tag);
        master
    }
}
// Heading
pub struct Heading {
    text: String,
    level: u8, 
}

impl Heading {
    pub fn new() -> Self {
        Heading { text: String::new(), level: 1 } 
    }
}
impl Renderable for Heading {
    fn render(&self) -> String {
        let mut master = String::new();
        let start_tag =  format!("<h{}>", self.level);
        let end_tag = format!("</h{}>", self.level);

        master.push_str(&start_tag);
        master.push_str(&self.text);
        master.push_str(&end_tag);
        master
    }
}

// Heading
pub struct HTML {
    items: Vec<Box<dyn Renderable>>,
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
        let start_tag =  "<html>";
        let end_tag = "</html>";

        master.push_str(&start_tag);
        for child in &self.items {
            let text = child.render();
            master.push_str(text.as_str());
        }
        master.push_str(&end_tag);
        master
    }
}
