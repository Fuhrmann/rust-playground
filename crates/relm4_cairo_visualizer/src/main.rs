use std::cell::RefCell;
use std::rc::Rc;

use relm4::gtk::cairo::LinearGradient;
use relm4::gtk::prelude::*;
use relm4::{gtk, Component, ComponentParts, ComponentSender, RelmApp};
use visualizer::Visualizer;

pub mod visualizer;

pub struct AppModel {
    // The number of bars we want to show
    bars: usize,
    // We use Rc (Reference Counted) here to allow multiple ownership of the data
    // This is necessary because both the AppModel and the drawing closure need access to bars_data
    // RefCell provides interior mutability, allowing us to mutate the Vec<u16> even when shared
    // This combination enables shared mutable state across different parts of our application
    bars_data: Rc<RefCell<Vec<u16>>>, // The cava data (smoothed by the visualizer)
    // Whether we should keep rendering in the DrawingArea
    should_draw: Rc<RefCell<bool>>,
}

#[derive(Debug)]
pub enum AppMsg {
    UpdateBarValues(Vec<u16>),
}

#[relm4::component(pub)]
impl Component for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = usize;
    type CommandOutput = ();

    view! {
        gtk::ApplicationWindow {
            set_title: Some("Simple Manual"),
            #[name="root"]
            gtk::DrawingArea {
                set_draw_func: {
                    // We need to clone the Rc<RefCell> so we can move it into the closure
                    // The closure will take ownership of the data and the should_draw flag
                    let bars_data = model.bars_data.clone();
                    let should_draw = model.should_draw.clone();
                    move |_, ctx, width, height| {
                        let area_width = width as f64;
                        let area_height = height as f64;

                        // `ctx` is a cairo context used for drawing on the surface
                        // `area_width` and `area_height` represent the dimensions of the DrawingArea
                        // Drawing occurs only when the `should_draw` flag is true
                        // The flag is set to false when there's no change in the visualizer data
                        // This optimization prevents unnecessary redrawing of the bars
                        if !*should_draw.borrow() {
                            ()
                        }

                        // Paint the background to dark
                        ctx.set_source_rgb(0.0, 0.0, 0.0);
                        ctx.paint().unwrap();

                        // Calculate the width of each bar
                        // Formula: bar_width = DrawingArea_width / number_of_bars
                        // Example: For 20 bars in an 800px wide area, each bar is 40px wide
                        // This ensures bars are evenly distributed across the available space
                        let stroke_width = 4.0f64;
                        let padding = 10.0;
                        let bar_width = area_width / model.bars as f64;

                        // Since we are using a RefCell, we need to borrow the data inside of it
                        let bars_data = bars_data.borrow();

                        // Iterate over the bars data, drawing each bar as a rectangle
                        // The index 'i' determines the bar's horizontal position, while 'bar_height' sets its vertical size
                        for (i, &bar_height) in bars_data.iter().enumerate() {
                            // Calculate the X position of each bar
                            // The X position is determined by the bar's index (i) multiplied by the bar width
                            // This ensures equal spacing between bars across the drawing area
                            // Example:
                            //   let bar_width = 50.0;
                            //   for i in 0..5 {
                            //       let x = i as f64 * bar_width;
                            //       println!("Bar {}: x = {}", i, x);
                            //   }
                            // Output:
                            //   Bar 0: x = 0.0
                            //   Bar 1: x = 50.0
                            //   Bar 2: x = 100.0
                            //   Bar 3: x = 150.0
                            //   Bar 4: x = 200.0
                            let x = (i as f64 * bar_width) + padding / 2.0;
                            let bar_width = bar_width - padding;

                            // Calculate the height of each bar
                            // The bar height is normalized by dividing the current value by the maximum possible value (u16::MAX = 65535)
                            // This ensures that the bar heights are proportional to their values and fit within the drawing area
                            let height = (bar_height as u64 * height as u64) / u16::MAX as u64;

                            // Calculate the Y position of the bar
                            // The Y position is determined by subtracting the bar's height from the drawing area's height
                            // This positions the bar from the bottom of the drawing area
                            // Example:
                            //   let area_height = 200.0;
                            //   let bar_height = 50.0;
                            //   let y = area_height - bar_height; // y = 150.0
                            // The bar would start at y = 150.0 and extend upwards to y = 200.0
                            // Note: (0,0) is at the top-left corner of the drawing area
                            // Increasing Y moves downward, while increasing height moves upward
                            let y = area_height - height as f64;

                            // Draw a stroke (border) around the bar
                            // Set the color for the stroke (light purple with some transparency)
                            ctx.set_source_rgba(0.8, 0.2, 1.0, 0.8);
                            ctx.set_line_width(stroke_width);

                            // Draw the rectangle for the stroke and apply the stroke
                            ctx.rectangle(x, y, bar_width, height as f64);
                            ctx.stroke().expect("Failed to stroke bar");

                            // Fill the bar with a gradient
                            let gradient = LinearGradient::new(x, y, x, y + height as f64);
                            gradient.add_color_stop_rgb(0.0, 0.1, 0.6, 0.8); // Top color (light blue)
                            gradient.add_color_stop_rgb(1.0, 0.0, 0.3, 0.5); // Bottom color (darker blue)
                            ctx.set_source(&gradient).expect("Failed to set gradient");

                            // Draw and fill the rectangle for the bar
                            ctx.rectangle(x, y, bar_width, height as f64);
                            ctx.fill().expect("Failed to fill bar");

                            // Add a shine effect at the top of the bar
                            let shine_height = height as f64 * 0.1; // 10% of bar height
                            let shine_gradient = LinearGradient::new(x, y, x, y + shine_height);
                            shine_gradient.add_color_stop_rgba(0.0, 1.0, 1.0, 1.0, 0.3); // White with 30% opacity
                            shine_gradient.add_color_stop_rgba(1.0, 1.0, 1.0, 1.0, 0.0); // Fully transparent
                            ctx.set_source(&shine_gradient).expect("Failed to set shine gradient");
                            ctx.rectangle(x, y, bar_width, shine_height);
                            ctx.fill().expect("Failed to add shine effect");

                            // Draw the current Y position on top of the rectangle
                            ctx.set_source_rgb(1.0, 1.0, 1.0); // White color for text
                            ctx.set_font_size(12.0);
                            let text = format!("y: {:.0}", y);
                            let extents = ctx.text_extents(&text).expect("Failed to get text extents");

                            // Calculate the center position for the text
                            // We start from the left edge of the bar (x) and add half the bar width
                            // Then we subtract half the text width to center it within the bar
                            let text_x = x + (bar_width - extents.width()) / 2.0;
                            let text_y = y - 5.0; // Position text slightly above the bar

                            // Move the drawing cursor to the specified (x, y) coordinates
                            // This sets the starting point for the next drawing operation (in this case, drawing text)
                            ctx.move_to(text_x, text_y);

                            // Draw the text on the canvas
                            ctx.show_text(&text).expect("Failed to draw text");

                            // Draw another text showing the bar's height
                            ctx.set_source_rgb(1.0, 1.0, 1.0);
                            ctx.set_font_size(12.0);
                            let text = format!("h: {:.0}", height);
                            let text_y = text_y - 10f64; // Position text slightly above the y text
                            ctx.move_to(text_x, text_y);
                            ctx.show_text(&text).expect("Failed to draw text");
                        }
                    }
                }
            }
        }
    }

    // Initialize the UI.
    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            bars: init,
            should_draw: Rc::new(RefCell::new(false)),
            bars_data: Rc::new(RefCell::new(vec![0_u16; init])),
        };

        // The sender is responsible for sending the data received from cava to the UI
        // Every time we receive data from the visualizer, we will send it to the sender
        // so it can be processed by the update_with_view function
        let clone = sender.clone();
        let bars = model.bars;
        relm4::spawn(async move {
            let rx = Visualizer::new(bars);

            // As long as we are receiving data from the visualizer, send it to the UI
            while let Ok(data) = rx.recv() {
                // let smoothed_data = visualizer.smooth_data(data);
                clone
                    .input_sender()
                    .send(AppMsg::UpdateBarValues(data))
                    .unwrap();
            }
        });

        // Render our widgest declared with the view! macro
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        _: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppMsg::UpdateBarValues(data) => {
                let mut should_draw = self.should_draw.borrow_mut();
                *should_draw = false; // Start by assuming no drawing needed

                let mut self_bar_values = self.bars_data.borrow_mut();

                // Iterate through the new data, updating bar values and setting should_draw flag
                // If any value changes, we need to redraw the entire visualization
                for (i, &new_value) in data.iter().enumerate() {
                    if self_bar_values[i] != new_value {
                        self_bar_values[i] = new_value;
                        if !*should_draw {
                            *should_draw = true;
                        }
                    }
                }

                if *should_draw {
                    widgets.root.queue_draw(); // Request a redraw if needed
                }
            }
        }
    }
}

pub fn main() {
    let app = RelmApp::new("fuhrmann.playground.relm4_audio_visualizer");
    app.run::<AppModel>(20);
}
