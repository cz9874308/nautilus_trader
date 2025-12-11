// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Real-time and test timers for use with `Clock` implementations.
//! 用于 `Clock` 实现的实时和测试定时器。

use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    num::NonZeroU64,
    rc::Rc,
};

use nautilus_core::{
    UUID4, UnixNanos,
    correctness::{FAILED, check_valid_string_utf8},
};
#[cfg(feature = "python")]
use pyo3::{Py, PyAny, Python};
use ustr::Ustr;

/// Creates a valid nanoseconds interval that is guaranteed to be positive.
/// 创建一个保证为正数的有效纳秒间隔。
///
/// # Panics
/// # 可能 panic 的情况
///
/// Panics if `interval_ns` is zero.
/// 如果 `interval_ns` 为零，则会 panic。
#[must_use]
pub fn create_valid_interval(interval_ns: u64) -> NonZeroU64 {
    NonZeroU64::new(std::cmp::max(interval_ns, 1)).expect("`interval_ns` must be positive")
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.common")
)]
/// Represents a time event occurring at the event timestamp.
/// 表示在事件时间戳发生的时间事件。
///
/// A `TimeEvent` carries metadata such as the event's name, a unique event ID,
/// and timestamps indicating when the event was scheduled to occur and when it was initialized.
/// `TimeEvent` 携带元数据，例如事件名称、唯一事件 ID，
/// 以及指示事件计划发生时间和初始化时间的时间戳。
pub struct TimeEvent {
    /// The event name, identifying the nature or purpose of the event.
    /// 事件名称，标识事件的性质或目的。
    pub name: Ustr,
    /// The unique identifier for the event.
    /// 事件的唯一标识符。
    pub event_id: UUID4,
    /// UNIX timestamp (nanoseconds) when the event occurred.
    /// 事件发生时的 UNIX 时间戳（纳秒）。
    pub ts_event: UnixNanos,
    /// UNIX timestamp (nanoseconds) when the instance was created.
    /// 实例创建时的 UNIX 时间戳（纳秒）。
    pub ts_init: UnixNanos,
}

impl TimeEvent {
    /// Creates a new [`TimeEvent`] instance.
    /// 创建一个新的 [`TimeEvent`] 实例。
    ///
    /// # Safety
    /// # 安全性
    ///
    /// Assumes `name` is a valid string.
    /// 假设 `name` 是有效的字符串。
    #[must_use]
    pub const fn new(name: Ustr, event_id: UUID4, ts_event: UnixNanos, ts_init: UnixNanos) -> Self {
        Self {
            name,
            event_id,
            ts_event,
            ts_init,
        }
    }
}

impl Display for TimeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}(name={}, event_id={}, ts_event={}, ts_init={})",
            stringify!(TimeEvent),
            self.name,
            self.event_id,
            self.ts_event,
            self.ts_init
        )
    }
}

/// Wrapper for [`TimeEvent`] that implements ordering by timestamp for heap scheduling.
/// 为 [`TimeEvent`] 的包装器，实现按时间戳排序以用于堆调度。
///
/// This newtype allows time events to be ordered in a priority queue (max heap) by their
/// timestamp while keeping [`TimeEvent`] itself clean with standard field-based equality.
/// Events are ordered in reverse (earlier timestamps have higher priority).
/// 此新类型允许时间事件在优先级队列（最大堆）中按其时间戳排序，
/// 同时保持 [`TimeEvent`] 本身具有基于字段的标准相等性。
/// 事件按相反顺序排序（较早的时间戳具有更高的优先级）。
#[repr(transparent)] // Guarantees zero-cost abstraction with identical memory layout
                      // 保证零成本抽象，具有相同的内存布局
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScheduledTimeEvent(pub TimeEvent);

impl ScheduledTimeEvent {
    /// Creates a new scheduled time event.
    /// 创建一个新的计划时间事件。
    #[must_use]
    pub const fn new(event: TimeEvent) -> Self {
        Self(event)
    }

    /// Extracts the inner time event.
    /// 提取内部时间事件。
    #[must_use]
    pub fn into_inner(self) -> TimeEvent {
        self.0
    }
}

impl PartialOrd for ScheduledTimeEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledTimeEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for max heap: earlier timestamps have higher priority
        other.0.ts_event.cmp(&self.0.ts_event)
    }
}

/// Callback type for time events.
/// 时间事件的回调类型。
pub enum TimeEventCallback {
    #[cfg(feature = "python")]
    Python(Py<PyAny>),
    Rust(Rc<dyn Fn(TimeEvent)>),
}

impl Clone for TimeEventCallback {
    fn clone(&self) -> Self {
        match self {
            #[cfg(feature = "python")]
            Self::Python(obj) => Self::Python(nautilus_core::python::clone_py_object(obj)),
            Self::Rust(cb) => Self::Rust(cb.clone()),
        }
    }
}

impl Debug for TimeEventCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "python")]
            Self::Python(_) => f.write_str("Python callback"),
            Self::Rust(_) => f.write_str("Rust callback"),
        }
    }
}

impl TimeEventCallback {
    /// Returns `true` if this is a Rust callback.
    /// 如果这是 Rust 回调，则返回 `true`。
    ///
    /// Rust callbacks use `Rc` internally and are NOT thread-safe.
    /// They must only be used with `TestClock`, never with `LiveClock`.
    /// Rust 回调在内部使用 `Rc` 且不是线程安全的。
    /// 它们只能与 `TestClock` 一起使用，绝不能与 `LiveClock` 一起使用。
    #[must_use]
    pub const fn is_rust(&self) -> bool {
        matches!(self, Self::Rust(_))
    }

    /// Invokes the callback for the given `TimeEvent`.
    /// 为给定的 `TimeEvent` 调用回调。
    ///
    /// # Panics
    /// # 可能 panic 的情况
    ///
    /// Panics if the underlying Python callback invocation fails (e.g., raises an exception).
    /// 如果底层 Python 回调调用失败（例如，引发异常），则会 panic。
    pub fn call(&self, event: TimeEvent) {
        match self {
            #[cfg(feature = "python")]
            Self::Python(callback) => {
                Python::attach(|py| {
                    callback.call1(py, (event,)).unwrap();
                });
            }
            Self::Rust(callback) => callback(event),
        }
    }
}

impl<F> From<F> for TimeEventCallback
where
    F: Fn(TimeEvent) + 'static,
{
    fn from(value: F) -> Self {
        Self::Rust(Rc::new(value))
    }
}

impl From<Rc<dyn Fn(TimeEvent)>> for TimeEventCallback {
    fn from(value: Rc<dyn Fn(TimeEvent)>) -> Self {
        Self::Rust(value)
    }
}

#[cfg(feature = "python")]
impl From<Py<PyAny>> for TimeEventCallback {
    fn from(value: Py<PyAny>) -> Self {
        Self::Python(value)
    }
}

// SAFETY: TimeEventCallback implements Send + Sync with the following invariants:
// - Python variant: Uses Py<PyAny> which is inherently Send + Sync (GIL acquired when needed)
// - Rust variant: Uses Rc<dyn Fn> which is NOT Send/Sync. This is only safe because:
//   * Rust callbacks are ONLY used with TestClock (single-threaded backtesting)
//   * LiveClock/LiveTimer MUST NOT use Rust callbacks - they will panic at runtime
//   * Python callbacks are used for all live/async timer contexts
//
// WARNING: Using TimeEventCallback::Rust with LiveClock is undefined behavior.
// The runtime check in LiveTimer enforces this invariant.
#[allow(unsafe_code)]
unsafe impl Send for TimeEventCallback {}
#[allow(unsafe_code)]
unsafe impl Sync for TimeEventCallback {}

#[repr(C)]
#[derive(Clone, Debug)]
/// Represents a time event and its associated handler.
/// 表示时间事件及其关联的处理器。
///
/// `TimeEventHandler` associates a `TimeEvent` with a callback function that is triggered
/// when the event's timestamp is reached.
/// `TimeEventHandler` 将 `TimeEvent` 与回调函数关联，当达到事件的时间戳时触发该回调函数。
pub struct TimeEventHandlerV2 {
    /// The time event.
    /// 时间事件。
    pub event: TimeEvent,
    /// The callable handler for the event.
    pub callback: TimeEventCallback,
}

impl TimeEventHandlerV2 {
    /// Creates a new [`TimeEventHandlerV2`] instance.
    #[must_use]
    pub const fn new(event: TimeEvent, callback: TimeEventCallback) -> Self {
        Self { event, callback }
    }

    /// Executes the handler by invoking its callback for the associated event.
    ///
    /// # Panics
    ///
    /// Panics if the underlying callback invocation fails (e.g., a Python callback raises an exception).
    pub fn run(self) {
        let Self { event, callback } = self;
        callback.call(event);
    }
}

impl PartialOrd for TimeEventHandlerV2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TimeEventHandlerV2 {
    fn eq(&self, other: &Self) -> bool {
        self.event.ts_event == other.event.ts_event
    }
}

impl Eq for TimeEventHandlerV2 {}

impl Ord for TimeEventHandlerV2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.event.ts_event.cmp(&other.event.ts_event)
    }
}

/// A test timer for user with a `TestClock`.
///
/// `TestTimer` simulates time progression in a controlled environment,
/// allowing for precise control over event generation in test scenarios.
///
/// # Threading
///
/// The timer mutates its internal state and should only be used from its owning thread.
#[derive(Clone, Copy, Debug)]
pub struct TestTimer {
    /// The name of the timer.
    pub name: Ustr,
    /// The interval between timer events in nanoseconds.
    pub interval_ns: NonZeroU64,
    /// The start time of the timer in UNIX nanoseconds.
    pub start_time_ns: UnixNanos,
    /// The optional stop time of the timer in UNIX nanoseconds.
    pub stop_time_ns: Option<UnixNanos>,
    /// If the timer should fire immediately at start time.
    pub fire_immediately: bool,
    next_time_ns: UnixNanos,
    is_expired: bool,
}

impl TestTimer {
    /// Creates a new [`TestTimer`] instance.
    ///
    /// # Panics
    ///
    /// Panics if `name` is not a valid string.
    #[must_use]
    pub fn new(
        name: Ustr,
        interval_ns: NonZeroU64,
        start_time_ns: UnixNanos,
        stop_time_ns: Option<UnixNanos>,
        fire_immediately: bool,
    ) -> Self {
        check_valid_string_utf8(name, stringify!(name)).expect(FAILED);

        let next_time_ns = if fire_immediately {
            start_time_ns
        } else {
            start_time_ns + interval_ns.get()
        };

        Self {
            name,
            interval_ns,
            start_time_ns,
            stop_time_ns,
            fire_immediately,
            next_time_ns,
            is_expired: false,
        }
    }

    /// Returns the next time in UNIX nanoseconds when the timer will fire.
    #[must_use]
    pub const fn next_time_ns(&self) -> UnixNanos {
        self.next_time_ns
    }

    /// Returns whether the timer is expired.
    #[must_use]
    pub const fn is_expired(&self) -> bool {
        self.is_expired
    }

    #[must_use]
    pub const fn pop_event(&self, event_id: UUID4, ts_init: UnixNanos) -> TimeEvent {
        TimeEvent {
            name: self.name,
            event_id,
            ts_event: self.next_time_ns,
            ts_init,
        }
    }

    /// Advance the test timer forward to the given time, generating a sequence
    /// of events. A [`TimeEvent`] is appended for each time a next event is
    /// <= the given `to_time_ns`.
    ///
    /// This allows testing of multiple time intervals within a single step.
    pub fn advance(&mut self, to_time_ns: UnixNanos) -> impl Iterator<Item = TimeEvent> + '_ {
        // Calculate how many events should fire up to and including to_time_ns
        let advances = if self.next_time_ns <= to_time_ns {
            (to_time_ns.as_u64() - self.next_time_ns.as_u64()) / self.interval_ns.get() + 1
        } else {
            0
        };
        self.take(advances as usize).map(|(event, _)| event)
    }

    /// Cancels the timer (the timer will not generate an event).
    ///
    /// Used to stop the timer before its scheduled stop time.
    pub const fn cancel(&mut self) {
        self.is_expired = true;
    }
}

impl Iterator for TestTimer {
    type Item = (TimeEvent, UnixNanos);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_expired {
            None
        } else {
            // Check if current event would exceed stop time before creating the event
            if let Some(stop_time_ns) = self.stop_time_ns
                && self.next_time_ns > stop_time_ns
            {
                self.is_expired = true;
                return None;
            }

            let item = (
                TimeEvent {
                    name: self.name,
                    event_id: UUID4::new(),
                    ts_event: self.next_time_ns,
                    ts_init: self.next_time_ns,
                },
                self.next_time_ns,
            );

            // Check if we should expire after this event (for repeating timers at stop boundary)
            if let Some(stop_time_ns) = self.stop_time_ns
                && self.next_time_ns == stop_time_ns
            {
                self.is_expired = true;
            }

            self.next_time_ns += self.interval_ns;

            Some(item)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use nautilus_core::UnixNanos;
    use rstest::*;
    use ustr::Ustr;

    use super::{TestTimer, TimeEvent};

    #[rstest]
    fn test_test_timer_pop_event() {
        let mut timer = TestTimer::new(
            Ustr::from("TEST_TIMER"),
            NonZeroU64::new(1).unwrap(),
            UnixNanos::from(1),
            None,
            false,
        );

        assert!(timer.next().is_some());
        assert!(timer.next().is_some());
        timer.is_expired = true;
        assert!(timer.next().is_none());
    }

    #[rstest]
    fn test_test_timer_advance_within_next_time_ns() {
        let mut timer = TestTimer::new(
            Ustr::from("TEST_TIMER"),
            NonZeroU64::new(5).unwrap(),
            UnixNanos::default(),
            None,
            false,
        );
        let _: Vec<TimeEvent> = timer.advance(UnixNanos::from(1)).collect();
        let _: Vec<TimeEvent> = timer.advance(UnixNanos::from(2)).collect();
        let _: Vec<TimeEvent> = timer.advance(UnixNanos::from(3)).collect();
        assert_eq!(timer.advance(UnixNanos::from(4)).count(), 0);
        assert_eq!(timer.next_time_ns, 5);
        assert!(!timer.is_expired);
    }

    #[rstest]
    fn test_test_timer_advance_up_to_next_time_ns() {
        let mut timer = TestTimer::new(
            Ustr::from("TEST_TIMER"),
            NonZeroU64::new(1).unwrap(),
            UnixNanos::default(),
            None,
            false,
        );
        assert_eq!(timer.advance(UnixNanos::from(1)).count(), 1);
        assert!(!timer.is_expired);
    }

    #[rstest]
    fn test_test_timer_advance_up_to_next_time_ns_with_stop_time() {
        let mut timer = TestTimer::new(
            Ustr::from("TEST_TIMER"),
            NonZeroU64::new(1).unwrap(),
            UnixNanos::default(),
            Some(UnixNanos::from(2)),
            false,
        );
        assert_eq!(timer.advance(UnixNanos::from(2)).count(), 2);
        assert!(timer.is_expired);
    }

    #[rstest]
    fn test_test_timer_advance_beyond_next_time_ns() {
        let mut timer = TestTimer::new(
            Ustr::from("TEST_TIMER"),
            NonZeroU64::new(1).unwrap(),
            UnixNanos::default(),
            Some(UnixNanos::from(5)),
            false,
        );
        assert_eq!(timer.advance(UnixNanos::from(5)).count(), 5);
        assert!(timer.is_expired);
    }

    #[rstest]
    fn test_test_timer_advance_beyond_stop_time() {
        let mut timer = TestTimer::new(
            Ustr::from("TEST_TIMER"),
            NonZeroU64::new(1).unwrap(),
            UnixNanos::default(),
            Some(UnixNanos::from(5)),
            false,
        );
        assert_eq!(timer.advance(UnixNanos::from(10)).count(), 5);
        assert!(timer.is_expired);
    }

    #[rstest]
    fn test_test_timer_advance_exact_boundary() {
        let mut timer = TestTimer::new(
            Ustr::from("TEST_TIMER"),
            NonZeroU64::new(5).unwrap(),
            UnixNanos::from(0),
            None,
            false,
        );
        assert_eq!(
            timer.advance(UnixNanos::from(5)).count(),
            1,
            "Expected one event at the 5 ns boundary"
        );
        assert_eq!(
            timer.advance(UnixNanos::from(10)).count(),
            1,
            "Expected one event at the 10 ns boundary"
        );
    }

    #[rstest]
    fn test_test_timer_fire_immediately_true() {
        let mut timer = TestTimer::new(
            Ustr::from("TEST_TIMER"),
            NonZeroU64::new(5).unwrap(),
            UnixNanos::from(10),
            None,
            true, // fire_immediately = true
        );

        // With fire_immediately=true, next_time_ns should be start_time_ns
        assert_eq!(timer.next_time_ns(), UnixNanos::from(10));

        // Advance to start time should produce an event
        let events: Vec<TimeEvent> = timer.advance(UnixNanos::from(10)).collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].ts_event, UnixNanos::from(10));

        // Next event should be at start_time + interval
        assert_eq!(timer.next_time_ns(), UnixNanos::from(15));
    }

    #[rstest]
    fn test_test_timer_fire_immediately_false() {
        let mut timer = TestTimer::new(
            Ustr::from("TEST_TIMER"),
            NonZeroU64::new(5).unwrap(),
            UnixNanos::from(10),
            None,
            false, // fire_immediately = false
        );

        // With fire_immediately=false, next_time_ns should be start_time_ns + interval
        assert_eq!(timer.next_time_ns(), UnixNanos::from(15));

        // Advance to start time should produce no events
        assert_eq!(timer.advance(UnixNanos::from(10)).count(), 0);

        // Advance to first interval should produce an event
        let events: Vec<TimeEvent> = timer.advance(UnixNanos::from(15)).collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].ts_event, UnixNanos::from(15));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Property-based testing
    ////////////////////////////////////////////////////////////////////////////////

    use proptest::prelude::*;

    #[derive(Clone, Debug)]
    enum TimerOperation {
        AdvanceTime(u64),
        Cancel,
    }

    fn timer_operation_strategy() -> impl Strategy<Value = TimerOperation> {
        prop_oneof![
            8 => prop::num::u64::ANY.prop_map(|v| TimerOperation::AdvanceTime(v % 1000 + 1)),
            2 => Just(TimerOperation::Cancel),
        ]
    }

    fn timer_config_strategy() -> impl Strategy<Value = (u64, u64, Option<u64>, bool)> {
        (
            1u64..=100u64,                    // interval_ns (1-100)
            0u64..=50u64,                     // start_time_ns (0-50)
            prop::option::of(51u64..=200u64), // stop_time_ns (51-200 or None)
            prop::bool::ANY,                  // fire_immediately
        )
    }

    fn timer_test_strategy()
    -> impl Strategy<Value = (Vec<TimerOperation>, (u64, u64, Option<u64>, bool))> {
        (
            prop::collection::vec(timer_operation_strategy(), 5..=50),
            timer_config_strategy(),
        )
    }

    #[allow(clippy::needless_collect)] // Collect needed for indexing and .is_empty()
    fn test_timer_with_operations(
        operations: Vec<TimerOperation>,
        (interval_ns, start_time_ns, stop_time_ns, fire_immediately): (u64, u64, Option<u64>, bool),
    ) {
        let mut timer = TestTimer::new(
            Ustr::from("PROP_TEST_TIMER"),
            NonZeroU64::new(interval_ns).unwrap(),
            UnixNanos::from(start_time_ns),
            stop_time_ns.map(UnixNanos::from),
            fire_immediately,
        );

        let mut current_time = start_time_ns;

        for operation in operations {
            if timer.is_expired() {
                break;
            }

            match operation {
                TimerOperation::AdvanceTime(delta) => {
                    let to_time = current_time + delta;
                    let events: Vec<TimeEvent> = timer.advance(UnixNanos::from(to_time)).collect();
                    current_time = to_time;

                    // Verify event ordering and timing
                    for (i, event) in events.iter().enumerate() {
                        // Event timestamps should be in order
                        if i > 0 {
                            assert!(
                                event.ts_event >= events[i - 1].ts_event,
                                "Events should be in chronological order"
                            );
                        }

                        // Event timestamp should be within reasonable bounds
                        assert!(
                            event.ts_event.as_u64() >= start_time_ns,
                            "Event timestamp should not be before start time"
                        );

                        assert!(
                            event.ts_event.as_u64() <= to_time,
                            "Event timestamp should not be after advance time"
                        );

                        // If there's a stop time, event should not exceed it
                        if let Some(stop_time_ns) = stop_time_ns {
                            assert!(
                                event.ts_event.as_u64() <= stop_time_ns,
                                "Event timestamp should not exceed stop time"
                            );
                        }
                    }
                }
                TimerOperation::Cancel => {
                    timer.cancel();
                    assert!(timer.is_expired(), "Timer should be expired after cancel");
                }
            }

            // Timer invariants
            if !timer.is_expired() {
                // Next time should be properly spaced
                let expected_interval_multiple = if fire_immediately {
                    timer.next_time_ns().as_u64() >= start_time_ns
                } else {
                    timer.next_time_ns().as_u64() >= start_time_ns + interval_ns
                };
                assert!(
                    expected_interval_multiple,
                    "Next time should respect interval spacing"
                );

                // If timer has stop time, check if it should be considered logically expired
                // Note: Timer only becomes actually expired when advance() or next() is called
                if let Some(stop_time_ns) = stop_time_ns
                    && timer.next_time_ns().as_u64() > stop_time_ns
                {
                    // The timer should expire on the next advance/iteration
                    let mut test_timer = timer;
                    let events: Vec<TimeEvent> = test_timer
                        .advance(UnixNanos::from(stop_time_ns + 1))
                        .collect();
                    assert!(
                        events.is_empty() || test_timer.is_expired(),
                        "Timer should not generate events beyond stop time"
                    );
                }
            }
        }

        // Final consistency check: if timer is not expired and we haven't hit stop time,
        // advancing far enough should eventually expire it
        if !timer.is_expired()
            && let Some(stop_time_ns) = stop_time_ns
        {
            let events: Vec<TimeEvent> = timer
                .advance(UnixNanos::from(stop_time_ns + 1000))
                .collect();
            assert!(
                timer.is_expired() || events.is_empty(),
                "Timer should eventually expire or stop generating events"
            );
        }
    }

    proptest! {
        #[rstest]
        fn prop_timer_advance_operations((operations, config) in timer_test_strategy()) {
            test_timer_with_operations(operations, config);
        }

        #[rstest]
        fn prop_timer_interval_consistency(
            interval_ns in 1u64..=100u64,
            start_time_ns in 0u64..=50u64,
            fire_immediately in prop::bool::ANY,
            advance_count in 1usize..=20usize,
        ) {
            let mut timer = TestTimer::new(
                Ustr::from("CONSISTENCY_TEST"),
                NonZeroU64::new(interval_ns).unwrap(),
                UnixNanos::from(start_time_ns),
                None, // No stop time for this test
                fire_immediately,
            );

            let mut previous_event_time = if fire_immediately { start_time_ns } else { start_time_ns + interval_ns };

            for _ in 0..advance_count {
                let events: Vec<TimeEvent> = timer.advance(UnixNanos::from(previous_event_time)).collect();

                if !events.is_empty() {
                    // Should get exactly one event at the expected time
                    prop_assert_eq!(events.len(), 1);
                    prop_assert_eq!(events[0].ts_event.as_u64(), previous_event_time);
                }

                previous_event_time += interval_ns;
            }
        }
    }
}
