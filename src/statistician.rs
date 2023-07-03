use core::intrinsics::sqrtf64;

/// An online algorithm for calculating mean and standard deviation.
///
/// # Algorithm
///
/// [Welford's Online Algorithm][1] is used.
///
/// # Resources
///
/// The memory requirement is constant.
///
/// [1]: https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm

#[derive(Clone, Copy, Debug)]
pub struct Statistician {
    size: f64,
    mean: f64,
    m2: f64,
    max: f64,
    min: f64,
}

impl Default for Statistician {
    fn default() -> Self {
        Statistician {
            size: 0.0,
            mean: 0.0,
            m2: 0.0,
            max: f64::MIN,
            min: f64::MAX,
        }
    }
}

impl core::fmt::Display for Statistician {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "\n--- Statistician Size: {} ---\nMean: {:5}, Max: {}, Min: {}\npstdev {:5}, sstdev {:5}\npsem: {:5}, ssem: {:5}\n",
            self.size(),
            self.mean(),
            self.max(),
            self.min(),
            self.pstdev(),
            self.sstdev(),
            self.psem(),
            self.ssem()
        )
    }
}

impl Statistician {
    /// Updates the data structure with a new observation `x`.
    pub fn update(&mut self, x: f64) {
        let delta = x - self.mean;

        self.size += 1.0;
        self.mean += delta / self.size;
        self.m2 += delta * (x - self.mean);
        self.max = if x > self.max { x } else { self.max };
        self.min = if x < self.min { x } else { self.min };
    }

    /// Returns the number of observations.
    #[must_use]
    pub const fn size(&self) -> f64 {
        self.size
    }

    /// Returns the mean.
    #[must_use]
    pub const fn mean(&self) -> f64 {
        self.mean
    }

    pub const fn max(&self) -> f64 {
        self.max
    }

    pub const fn min(&self) -> f64 {
        self.min
    }

    /// Returns the population standard deviation.
    #[must_use]
    pub fn pstdev(&self) -> f64 {
        if self.size > 1.0 {
            let variance = self.m2 / self.size;
            // variance.sqrt()
            unsafe { sqrtf64(variance) }
        } else {
            0.0
        }
    }

    /// Returns the sample standard deviation.
    #[must_use]
    pub fn sstdev(&self) -> f64 {
        if self.size > 1.0 {
            let variance = self.m2 / (self.size - 1.0);
            // variance.sqrt()
            unsafe { sqrtf64(variance) }
        } else {
            0.0
        }
    }

    /// Returns the population standard error of the mean.
    #[must_use]
    pub fn psem(&self) -> f64 {
        self.pstdev() / unsafe { sqrtf64(self.size) } //self.size.sqrt()
    }

    /// Returns the sample standard error of the mean.
    #[must_use]
    pub fn ssem(&self) -> f64 {
        self.sstdev() / unsafe { sqrtf64(self.size) }
    }
}