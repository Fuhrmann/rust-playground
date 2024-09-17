// Lets define a trait that will add a method to draw a widget
// Every widget that implements this trait is saying that
// it can be drawn on the screen
trait Widget {
    fn draw(&self);
}

// Basically what we are saying here is that for a struct to implement this trait
// it must also implement the Widget trait. Self:Widget is a trait bound.
trait ConfigurableWidget
where
    Self: Widget,
{
    // Here we define an associated type that will be used to
    // return the configuration of the widget. This way we can
    // have a method that returns multiple types depending on the
    // type of the widget.
    type ConfigType;
    fn config(&self) -> Self::ConfigType;
}

// Here we define our network status widget
// that can be draw on the screen
struct NetworkStatusWidget {}

#[allow(dead_code)]
struct NetworkStatusConfig {
    ssid: String,
    password: String,
}

// Now, to indicate tha the NetworkStatusWidget can be drawn on
// the screen we need to implement the Widget trait for it.
impl Widget for NetworkStatusWidget {
    // And now we must implement the draw method
    fn draw(&self) {
        println!("Drawing network status widget");
    }
}

impl ConfigurableWidget for NetworkStatusWidget {
    type ConfigType = NetworkStatusConfig;

    fn config(&self) -> Self::ConfigType {
        NetworkStatusConfig {
            ssid: "my_ssid".to_string(),
            password: "my_password".to_string(),
        }
    }
}

// We also have a battery widget
struct BatteryStatusWidget {}
#[allow(dead_code)]
struct BatteryStatusConfig {
    level: u8,
}

// And we implement the Widget trait for it
impl Widget for BatteryStatusWidget {
    fn draw(&self) {
        println!("Drawing battery status widget");
    }
}

// And we implement the ConfigurableWidget trait for it
impl ConfigurableWidget for BatteryStatusWidget {
    type ConfigType = BatteryStatusConfig;

    fn config(&self) -> Self::ConfigType {
        BatteryStatusConfig { level: 100 }
    }
}

// Now we want to create a status bar that can hold a single widget
// We can use a generic type parameter to indicate that the
// widget must implement the Widget trait.
struct StatusBarSingleWidget<T: Widget> {
    widget: T,
}

// Now, we want create a status bar that can hold multiple widgets
// But we need to use a vector to store the widgets. Since we can't
// have a vector of different types because it requires that all of
// its element occupy the same amount of space, we need to use a
// Box to store the widgets, so we can store different types.
struct StatusBar {
    widgets: Vec<Box<dyn Widget>>,
}

// We can create a widget drawer that will draw all the widgets
// By using the dyn keyword we are saying that the Widget trait
// is a trait object. This means that we can accept any type that
// implements the Widget trait
struct WidgetDrawer {}
impl WidgetDrawer {
    fn draw_widget(widget: &dyn Widget) {
        widget.draw();
    }
}

pub fn run() {
    // We can add either the network widget or the battery widget
    // Since both implement the Widget trait
    let network_widget = NetworkStatusWidget {};
    let status_bar_single = StatusBarSingleWidget {
        widget: network_widget,
    };
    status_bar_single.widget.draw();
    status_bar_single.widget.config();

    // We can also add both widgets to the status bar
    let boxed_battery_widget = Box::new(BatteryStatusWidget {});
    let boxed_network_widget = Box::new(NetworkStatusWidget {});
    let status_bar = StatusBar {
        widgets: vec![boxed_network_widget, boxed_battery_widget],
    };

    // And draw all the widgets in the status bar
    for widget in status_bar.widgets.iter() {
        WidgetDrawer::draw_widget(widget.as_ref());
    }
}
