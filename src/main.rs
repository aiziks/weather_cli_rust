use std::io;
use serde::{Deserialize};
use colored::*;
use dotenvy::dotenv;


// struct to deserialize the JSON response from OpenWeather API
#[derive(Deserialize , Debug)]
struct WeatherResponse{
    weather : Vec<Weather>,
    main: Main,
    wind: Wind,
    name : String,
}

// struct to represent weeather description
#[derive(Deserialize , Debug)]
struct Weather{
    description : String,
}

//struct to represent the main weeather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp : f64,
    humidity:f64,
    pressure : f64
}


// struct to represent wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed : f64,
}

// function to get the weather information from the OpenWeatherMap API
fn  get_weather_info(city:&str , country_code:&str, api_key:&str)
 -> Result<WeatherResponse, Box<dyn std::error::Error > >
//    ->  Result<() ,  reqwest::Error>
 {
    let url: String = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}", city, country_code , api_key );
    // let url: String = format!("http://api.weatherapi.com/v1/current.json?q={}",city);
    let response = reqwest::blocking::get(url)?;
    // println!("Response :: {:?} ", response );
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}


// fundtion to dispay the weather information
fn display_weather_info( response: &WeatherResponse ) {
    // Extract the information from the response
    let description = &response.weather[0].description;
    let temprature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    // foratting weather information into a string
    let weather_text = format!(
        "Weather in {}: {} {}
            > Temrature: {:.1} C
            > Humidity: {:.1} %
            > Pressure: {:.1} hpa
            > Wind Speed: {:.1} m/s",
            response.name,
            description,
            // this is a function to get emoji
            get_temp_emoji(temprature),
            temprature,
            humidity,
            pressure,
            wind_speed
    );

    
    let weather_text_colored  = match description.as_str() {
            "clear sky" => weather_text.bright_yellow(),
            "shower rain" | "rain" => weather_text.bright_cyan(),
            _ => weather_text.normal()
    };
    
    // print the colored weather information
    println!("{}", weather_text_colored );


}

    fn get_temp_emoji(temprature: f64) -> &'static str {
        if temprature < 0.0 {
            "❄️"
        }else if temprature >=0.0 && temprature <10.0 {
            "🌨️"
        }else if temprature >= 10.0 && temprature < 20.0 {
            " ⛅"
        }
        else if temprature >= 20.0 && temprature < 30.0 {
            "☀️"
        }else{
            "🔥"
        }
    }



    

fn main () {
    dotenv().ok();
    
    println!("{}","welcome to weather cli app {Weather Station}".bright_yellow() );

    loop {
        // Reading city
        println!("{}","Please enter the name of the city: ".bright_green() );
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input!");
        let city: &str = city.trim();

        // Reading country code
        println!("{}", "Please enter the country code (e.g US for United States) :".bright_green() );
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input");
        let country_code = country_code.trim();



        // Openweather API key
        let api_key = std::env::var("API_KEY").unwrap();

        // calling function to fetch weater information
        match get_weather_info( &city, &country_code, api_key.as_str() ) {
            Ok( response) => {
                display_weather_info( &response); //displays weather information
            },
            Err(error ) => {
                eprintln!("Error : {} ", error ); //printing error message in case of failure
            },
        }

        println!("{}", "Do you want to search for weather in another city? (yes/no):".bright_green()); 
        // prompting the user to continue of exit
        let mut input  = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input"); // reading user input for continuation
        let  input = input.trim().to_lowercase();

        if input != "yes"{
            println!("Thank you for using our software ");
            break; // Exiting the loop of user don't want to continue or chose no
        }


    }
}
