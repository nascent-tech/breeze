use super::monitor_cache::MonitorCache;
use breeze_ports::{Display, DisplayEnumerationPort};

pub struct TauriDisplays {
    cache: MonitorCache,
}

impl TauriDisplays {
    pub fn new(cache: MonitorCache) -> Self {
        TauriDisplays { cache }
    }
}

impl DisplayEnumerationPort for TauriDisplays {
    fn displays(&self) -> Vec<Display> {
        self.cache.snapshot()
    }
}
