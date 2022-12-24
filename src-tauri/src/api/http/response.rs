pub struct Response<T>(T);

impl<T> Response<T> {
    pub(super) fn new(res: T) -> Self {
        Self(res)
    }

    pub fn map<F, O>(self, mut f: F) -> O
      where F: FnMut(T) -> O {
        f(self.0)
    }

    pub fn map_future<F, Fut>(self, mut f: F) -> Fut 
      where Fut: std::future::IntoFuture,
            F: FnMut(T) -> Fut {
        f(self.0)
    }

    pub fn peeking<F, P>(&self, f: F) -> P
      where F: FnOnce(&T) -> P {
        f(&self.0)
    }
}