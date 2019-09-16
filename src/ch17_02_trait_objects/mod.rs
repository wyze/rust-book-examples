trait Draw {
    fn draw(&self);
}

struct Button {
    height: u32,
    label: String,
    width: u32,
}

impl Draw for Button {
    fn draw(&self) {
        // Draw it
    }
}

struct SelectBox {
    height: u32,
    options: Vec<String>,
    width: u32,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Draw it
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub fn run() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
                width: 75,
            }),
            Box::new(Button {
                height: 10,
                label: String::from("Ok"),
                width: 50,
            }),
        ],
    };

    screen.run();
}
