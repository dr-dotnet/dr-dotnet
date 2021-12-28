use crate::{
    ffi::{COR_PRF_HIGH_MONITOR, COR_PRF_MONITOR, HRESULT},
    CorProfilerInfo4, EventMask2,
};

pub trait CorProfilerInfo5: CorProfilerInfo4 {
    fn get_event_mask_2(&self) -> Result<EventMask2, HRESULT>;
    fn set_event_mask_2(
        &self,
        events_low: COR_PRF_MONITOR,
        events_high: COR_PRF_HIGH_MONITOR,
    ) -> Result<(), HRESULT>;
}
