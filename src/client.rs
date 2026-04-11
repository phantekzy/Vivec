use std::time::Duration;

use crate::interceptor::Interceptors;

pub struct Xsus {
    pub base_url: String,
    pub timeout: Duration,
    pub interceptors: Interceptors,
}
