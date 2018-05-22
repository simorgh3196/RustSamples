pub mod client;

pub mod network;

pub mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            ::outermost::middle_secret_function()
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
//    outermost::middle_secret_function();
//    outermost::inside::inner_function();
//    outermost::inside::secret_function();
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

//use TrafficLight::{Red, Yellow};
use TrafficLight::*;

fn main() {
    nested_modules();

//    let red = Red;
//    let yellow = Yellow;
//    let green = TrafficLight::Green;

    let red = Red;
    let yellow = Yellow;
    let green = Green;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::client::connect();
    }
}
