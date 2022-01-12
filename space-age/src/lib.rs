#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

impl Duration {
    pub fn as_earth_years(&self) -> f64 {
        self.0 as f64 / 31556952.0 
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet {
    ($struct: tt, $ratio:expr) => {
        pub struct $struct;

        impl Planet for $struct {
            fn years_during(d: &Duration) -> f64 {
                d.as_earth_years() * $ratio
            }
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 0.2408467);
planet!(Uranus,  84.016846);
planet!(Neptune, 164.79132);