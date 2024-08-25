// This enum represents the widgets that can be drawn on the screen
enum Widget {
    NetworkStatus(NetworkConfig),
    BatteryStatus(BatteryConfig),
}

// This trait add a method so that we can create a widget controller
// Each widget struct will implement this trait so it can create itself
trait WidgetFactory<T> {
    fn create_widget(config: T) -> Box<dyn WidgetController>;
}

// This allow us to create each widget that our application supports
impl Widget {
    fn create_widget(&self) -> Box<dyn WidgetController> {
        match self {
            Widget::NetworkStatus(config) => NetworkWidget::create_widget(config.clone()),
            Widget::BatteryStatus(config) => BatteryWidget::create_widget(config.clone()),
        }
    }
}

// This is what our status bar will be holding in a vector
// The widgets that implement this trait can be drawn on the screen
trait WidgetController {
    fn draw(&self);
}

struct StatusBar {
    controllers: Vec<Box<dyn WidgetController>>,
}

struct NetworkWidget {
    config: NetworkConfig,
}

#[derive(Clone)]
struct NetworkConfig {
    ssid: String,
    password: String,
}

impl WidgetFactory<NetworkConfig> for NetworkWidget {
    fn create_widget(config: NetworkConfig) -> Box<dyn WidgetController> {
        Box::new(NetworkWidget::from(config))
    }
}

impl WidgetController for NetworkWidget {
    fn draw(&self) {
        println!("Drawing network widget");
        println!("SSID: {}", self.config.ssid);
        println!("Password: {}", self.config.password);
    }
}

impl From<NetworkConfig> for NetworkWidget {
    fn from(config: NetworkConfig) -> Self {
        NetworkWidget { config }
    }
}

struct BatteryWidget {
    config: BatteryConfig,
}

#[derive(Clone)]
struct BatteryConfig {
    level: u8,
}

impl WidgetFactory<BatteryConfig> for BatteryWidget {
    fn create_widget(config: BatteryConfig) -> Box<dyn WidgetController> {
        Box::new(BatteryWidget::from(config))
    }
}

impl WidgetController for BatteryWidget {
    fn draw(&self) {
        println!("Drawing battery widget");
        println!("Battery level: {}%", self.config.level);
    }
}

impl From<BatteryConfig> for BatteryWidget {
    fn from(config: BatteryConfig) -> Self {
        BatteryWidget { config }
    }
}

pub fn run() {
    // Lets say we got a list of widgets that was loaded by parsing a configuration file.
    // Now we have to draw them on the screen by iterating over the list and creating
    // the widget controllers for each widget. We want to "hold" the widget controllers
    // so they dont drop out of scope and we can draw them on the screen later.
    let widgets = vec![
        Widget::NetworkStatus(NetworkConfig {
            ssid: "my_ssid".to_string(),
            password: "my_password".to_string(),
        }),
        Widget::BatteryStatus(BatteryConfig { level: 100 }),
    ];

    // We create a status bar that will hold all the widgets controllers
    let mut status_bar = StatusBar {
        controllers: vec![],
    };

    // We iterate over the widgets and create the widget controllers
    // by calling the associated method create_widget, which each
    // widget inherits by implementing the WidgetFactory trait
    for widget in widgets {
        let controller = widget.create_widget();
        status_bar.controllers.push(controller);
    }

    // Finally we draw all the widgets on the screen
    for widget in status_bar.controllers {
        widget.draw();
    }
}
