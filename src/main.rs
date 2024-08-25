use box_dyn_traits::Config;

mod atomic_counter;
mod bounded_channel;
mod box_dyn_traits;
mod trait_bounds;
mod trait_enum;

fn main() {
    // bounded_channel::std_bounded_channel(2);

    // atomic_counter::atomic_counter();

    // box_dyn_traits::init(Config {
    //     weather_service: "openweather".to_string(),
    // });

    //trait_bounds::run();

    trait_enum::run();
}
