# EWMA: compute exponential moving averages without introducing drift

This crate provides an abstraction to compute an [exponential moving
average](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average) over evenly or non-evenly spaced timeseries.


## Usage
```
use ewma::{EWMA, Smoothing};

let datapoints: Vec<f64> = vec![10.3, 11.9, -1.33, 2.0];

// Apply an exponential moving average with alpha=0.5
// on a stream of evenly spaced values:
let mut risk = EWMA::new(0.6f64, Smoothing::Static);
for datapoint in datapoints.iter() {
    risk.add(datapoint)
}

println!("risk score: {}", risk.value());

```

## Smoothing variants

### `Smoothing::Static`

If your timeseries is evenly spaced, you can simply use a static smoothing factor (aka. "alpha"). Every value is aggregated into a metric of the form: $E_n := \alpha * y_n + (1 - \alpha) * E_{n-1}$ where $y_n$ is the n-th observation and $E_k$ the rolling metric at step $k$.

#### Example of an evenly spaced timeseries:

A weather station reports the local temperature every hour. We have a sample for every hour. The delay between two points is always one hour.

### `Smoothing::Dynamic`

If the gap between two points varies,

$E_n := \exp(\frac{-\Delta_{t_n}}{\tau}) * y_n + (1-\exp(\frac{-\Delta_{t_n}}{\tau}))*E_{n-1}$


## Supporting Kernel smoothing methods

$\text{Soon}^{tm}$
