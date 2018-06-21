use lang::TimeSec;

/// time between current frame and last frame
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Timing {
    pub delta_time: TimeSec,
    pub last_frame: TimeSec,
}
