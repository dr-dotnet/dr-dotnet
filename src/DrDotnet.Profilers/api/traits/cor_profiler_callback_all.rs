#![allow(unused_variables)]
use crate::traits::*;

pub trait CorProfilerCallbackAll :       
  CorProfilerCallback9 
+ CorProfilerCallback8 
+ CorProfilerCallback7
+ CorProfilerCallback6
+ CorProfilerCallback5
+ CorProfilerCallback4
+ CorProfilerCallback3
+ CorProfilerCallback2
+ CorProfilerCallback1 {

}

impl<T> CorProfilerCallbackAll for T 
    where T: CorProfilerCallback1 + CorProfilerCallback2 + CorProfilerCallback3 + CorProfilerCallback4 + CorProfilerCallback5 + CorProfilerCallback6 + CorProfilerCallback7 + CorProfilerCallback8 + CorProfilerCallback9 {
    // Nothing to implement, since T already supports the other traits.
    // It has the functions it needs already
}