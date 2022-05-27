//! # EMWA: compute exponential moving averages without introducing drift
//! EMWA provides an abstraction to compute [exponential moving
//! averages](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average) over evenly
//! and non-evenly spaced timeseries.
//!
//! ```
//! use emwa_rs::{EMWA, Smoothing};
//!
//! let datapoints: Vec<f64> = vec![10.3, 11.9, -1.33, 2.0];
//!
//! // Apply an exponential moving average with alpha=0.5
//! // on a stream of evenly spaced values:
//! let mut risk = EMWA::new(0.6f64, Smoothing::Static);
//! for datapoint in datapoints.iter() {
//!     risk.add(datapoint)
//! }
//!
//! println!("risk score: {}", risk.value());
//!
//! ```

// #![warn(missing_docs)]

/// Errors that are specific to computing EMWAs
#[derive(Debug)]
pub enum Error {
    /// Applying a static operator to a dynamic strategy (and vice-versa).
    AlgoMismatch,
    /// Input datapoints older than current clock state (i.e stale)
    StaleData,
}

/// An abstraction representing an exponential moving average with a smoothing strategy.
///
/// # What is an EMWA?
///
/// An exponential moving average is used to summarize the average value of a time series over an
/// observation period, with a bias towards recent or older values depending on the choice of
/// `alpha`.
///
/// # Smoothing strategy
///
/// A smoothing strategy is a method to compute the smoothing factor (aka. alpha) which determines
/// how much weight is assigned to a new value vs. the historical data.
///
/// This crates ships with two smoothing strategies, as defined in the
/// [Smoothing](enum.Smoothing.html) enum:
///
/// - `Static`: the same weights are assigned to new and old datapoints respectively
/// - `Dynamic`: the weights applied to new data and accumulated values varies as a function of
/// time. Typically, this would be the case because the values in your time series are not sampled
/// at the same frequency.
/// - `Custom`: wip.
///
/// ## Evenly spaced time series
///
/// If your time series has evenly spaced datapoints, you want to use a `Smoothing::Static`
/// strategy.
///
/// ## Unevenly spaced time series
///
/// If your time series has unevenly spaced datapoints (i.e. the time between two observations
/// varies), you want to use a `Smoothing::Dynamic` strategy.
///
/// # How to pick a smoothing factor?
///
/// TODO: write guide + add plot to make my point
///
/// # Can I use a custom smoothing strategy.
/// Soon, inch'allah.
pub struct EMWA {
    /// smoothing factor (aka. alpha)
    alpha: f64,
    /// current value of the EMWA
    value: f64,
    /// number of observations seen
    datapoints: u32,
    /// internal clock for dynamic smoothing
    time: f64,
    /// smoothing strategy applied
    strategy: Smoothing,
}

/// The type of smoothing strategy that is applied to the time series.
pub enum Smoothing {
    /// A static smoothing factor (evenly spaced datapoints)
    Static,
    /// A dynamic smoothing factor (unevenly spaced datapoints)
    Dynamic,
}

impl EMWA {
    pub fn new(alpha: f64, strategy: Smoothing) -> Self {
        EMWA {
            alpha,
            value: 0f64,
            datapoints: 0,
            time: 0f64,
            strategy,
        }
    }

    /// Adds a datapoint to an EMWA with a `Smoothing::Static` strategy (evenly spaced
    /// observations)
    pub fn add(&mut self, data: f64) -> Result<f64, Error> {
        if let Smoothing::Dynamic = self.strategy {
            return Err(Error::AlgoMismatch);
        }

        self.datapoints += 1;
        if self.datapoints == 1 {
            self.value = data;
            Ok(self.value)
        } else {
            let new_value = self.alpha * data + (1f64 - self.alpha) * self.value;
            self.value = new_value;
            Ok(self.value)
        }
    }

    /// Computes the smoothing factor for a dynamic strategy
    fn compute_alpha(&self, time: f64) -> Result<f64, Error> {
        let diff = time - self.time;
        if diff < 0f64 {
            Err(Error::StaleData)
        } else {
            Ok((-diff / self.datapoints as f64).exp())
        }
    }

    /// Adds a datapoint to an EMWA with a `Smoothing::Dynamic` strategy (unevenly spaced ts)
    pub fn add_with_time(&mut self, data: f64, time: f64) -> Result<f64, Error> {
        if let Smoothing::Static = self.strategy {
            return Err(Error::AlgoMismatch);
        }

        self.datapoints += 1;
        if self.datapoints == 1 {
            self.value = data;
            self.time = time;
        } else {
            let new_alpha = self.compute_alpha(time)?;
            let new_value = new_alpha * data + (1f64 - new_alpha) * self.value;
            self.time = time;
            self.value = new_value;
        }

        Ok(self.value)
    }

    /// Returns the current value
    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::EMWA;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    /// When the smoothing factor is constant and equal to 1
    /// The rolling EMWA is always equal to the latest observed value.
    fn smoothing_factor_one() {
        let mut rolling = EMWA::new(1f64, crate::Smoothing::Static);
        for i in 1..100 {
            rolling.add(i as f64).unwrap();
        }
        assert_eq!(rolling.value(), 99f64)
    }

    #[test]
    fn smoothing_factor_zero() {
        let mut rolling = EMWA::new(0f64, crate::Smoothing::Static);
    }
}
