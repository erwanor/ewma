/*
 *
 * emwa(smoothing factor)
 *
 * emwa = EMWA(0.5)
 *
 * emwa.add(something_var)
 * emwa.add(something_var)
 * emwa.add(something_var)
 * emwa.add(something_var)
 * ...
 *
 * add( decimal )
 * value () decimal
 *
 *
 */

pub struct EMWA {
    alpha: f64,
    value: f64,
    datapoints: u32,
}

impl EMWA {
    pub fn new(alpha: f64) -> Self {
        EMWA {
            alpha,
            value: 0f64,
            datapoints: 0,
        }
    }

    pub fn add(&mut self, data: f64) -> f64 {
        if self.datapoints == 0 {
            self.value = data;
            self.value
        } else {
            let new_value = self.alpha * data + (1f64 - self.alpha) * self.value;
            self.value = new_value;
            self.value
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
