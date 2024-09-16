// First we define a configuration struct that will have the name of the weather service
// Lets say we have two weather services: OpenWeather and WttrIn
// And the user can configure which one to use in a configuration file (e.g. config.toml)
pub struct Config {
    pub weather_service: String,
}

// Then we define a trait that will have a method to get the weather
// The trait will be implemented by all the weather services
pub trait WeatherService {
    fn get_weather(&self) -> String;
}

// We define two structs that will implement the WeatherService trait
// The OpenWeather struct will have an API key
pub struct OpenWeather {
    api_key: String,
}

impl WeatherService for OpenWeather {
    fn get_weather(&self) -> String {
        format!("Loading weather from OpenWeather using {:?}", self.api_key)
    }
}

// The WttrIn struct will not have any fields
pub struct WttrIn;

impl WeatherService for WttrIn {
    fn get_weather(&self) -> String {
        format!("Loading weather from WttrIn")
    }
}

pub fn run(config: Config) {
    // This match statement will create a new instance of the weather service
    // We can only make this match work if we use Box<dyn WeatherService>
    // Since trait types are unsized, we need to use a pointer to the trait object
    // Or the compiler will complain that the match arms have incompatible types
    // Since in one arm we are returning OpenWeather and in the other WttrIn
    let service: Box<dyn WeatherService> = match config.weather_service.as_str() {
        "openweather" => Box::new(OpenWeather {
            api_key: "123".to_string(),
        }),
        "wttrin" => Box::new(WttrIn),
        _ => panic!("Unknown weather service"),
    };

    println!("{}", service.get_weather());
}
