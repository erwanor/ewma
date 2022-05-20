#[derive(Debug)]
pub enum EMWAError {
    Kind,
    Compute,
    Time,
}

pub struct EMWA {
    alpha: f64,
    value: f64,
    datapoints: u32,
    time: f64,
    kind: Alpha,
}

pub enum Alpha {
    Static,
    Dynamic,
}

impl EMWA {
    pub fn new(alpha: f64, kind: Alpha) -> Self {
        EMWA {
            alpha,
            value: 0f64,
            datapoints: 0,
            time: 0f64,
            kind,
        }
    }

    pub fn add(&mut self, data: f64) -> Result<f64, EMWAError> {
        if let Alpha::Dynamic = self.kind {
            return Err(EMWAError::Kind);
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

    fn compute_alpha(&self, time: f64) -> Result<f64, EMWAError> {
        let diff = time - self.time;
        if diff < 0f64 {
            Err(EMWAError::Time)
        } else {
            Ok((-diff / self.datapoints as f64).exp())
        }
    }

    pub fn add_with_time(&mut self, data: f64, time: f64) -> Result<f64, EMWAError> {
        if let Alpha::Static = self.kind {
            return Err(EMWAError::Kind);
        }

        self.datapoints += 1;
        if self.datapoints == 1 {
            self.value = data;
            Ok(self.value)
        } else {
            let new_alpha = self.compute_alpha(time)?;
            let new_value = new_alpha * data + (1f64 - new_alpha) * self.value;
            self.value = new_value;
            Ok(self.value)
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
