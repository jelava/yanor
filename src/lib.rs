use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt,
    fmt::{
        Debug,
        Formatter
    }
};

/// Any entity that needs to be processed in the update queue needs to implement the Updatable trait
pub trait Updatable {
    /// Update the entity. The return value is None if the entity should never be updated again
    /// (i.e. it is inactive or otherwise no longer exists), otherwise it should be Some(num), where num
    /// is a u32 indicating the number of ticks until the next update for this entity.
    ///
    /// It is (almost?) never necessary to call this directly; instead it should be called only via
    /// UpdateQueue.update, which will always check whether an updatable is dead before updating.
    fn update(&mut self) -> Option<u32>;

    /// Determine whether the entity is active (i.e. should be updated).
    fn is_active(&self) -> bool;

    /// Mark the entity as inactive, meaning that it should no longer be updated and will be ignored
    /// by UpdateQueues.
    fn deactivate(&mut self);
}

/// This is just a wrapper around an updatable that also stores the time when its next update is scheduled.
struct UpdateInfo<'a> {
    time: u32,
    updatable: &'a mut dyn Updatable
}

impl<'a> PartialEq for UpdateInfo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<'a> Eq for UpdateInfo<'a> { }

impl<'a> PartialOrd for UpdateInfo<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for UpdateInfo<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;

        match self.time.cmp(&other.time) {
            Greater => Less,
            Less => Greater,
            Equal => Equal
        }
    }
}

/// An UpdateQueue is responsible for handling updates of any entity added to it. It also makes sure
/// the entities it updates are kept ordered so that updates occur in the order that they need to occur.
pub struct UpdateQueue<'a> {
    queue: BinaryHeap<UpdateInfo<'a>>
}

impl<'a> UpdateQueue<'a> {
    /// Create an empty update queue.
    pub fn new() -> Self {
        UpdateQueue {
            queue: BinaryHeap::new()
        }
    }

    /// Add an updatable entity to the queue. The `time` parameter indicates the scheduled time of
    /// its first update.
    pub fn push(&mut self, time: u32, updatable: &'a mut dyn Updatable) {
        self.queue.push(UpdateInfo { time, updatable });
    }

    /// This is arguably the most important function in Yanor. It is responsible for processing the next
    /// updatable entity in the update queue (kind of a circular definition). In other words, it makes
    /// the game simulation move one very small step forward (updating a single entity). Any interface for
    /// Yanor will need to call this function a lot, probably inside of some other loop that also updates
    /// the display of the game state to reflect any changes caused by this function.
    ///
    /// If there are any active entities left in the queue, it will process whichever one of those entities
    /// is scheduled for the earliest time and return Some(t), where t is the scheduled time for that update.
    /// Otherwise it will return None, meaning that no more active entities are left in the queue. That
    /// should not normally happen for any reason other than the end of the game (either due to winning or dying).
    pub fn update(&mut self) -> Option<u32> {
        let mut info = self.queue.pop()?;

        while !info.updatable.is_active() {
            info = self.queue.pop()?;
        }

        let now = info.time;

        if let Some(dt) = info.updatable.update() {
            info.time += dt;
            self.queue.push(info);
        }

        Some(now)
    }
}

impl<'a> Debug for UpdateQueue<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for info in &self.queue {
            f.debug_struct("UpdateInfo")
                .field("time", &info.time)
                //.field("updatable", info.updatable)
                .finish()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Updatable, UpdateQueue};

    struct Dummy1 {
        update_count: u32,
        max_updates: u32
    }

    struct Dummy2 {
        has_updated: bool
    }

    impl Updatable for Dummy1 {
        fn update(&mut self) -> Option<u32> {
            self.update_count += 1;

            if self.update_count <= self.max_updates {
                Some(2)
            } else {
                None
            }
        }

        fn is_active(&self) -> bool {
            true
        }

        fn deactivate(&mut self) {
            unimplemented!()
        }
    }

    impl Updatable for Dummy2 {
        fn update(&mut self) -> Option<u32> {
            if self.has_updated {
                None
            } else {
                self.has_updated = true;
                Some(25)
            }
        }

        fn is_active(&self) -> bool {
            true
        }

        fn deactivate(&mut self) {
            unimplemented!()
        }
    }

    #[test]
    fn update_queue_basic() {
        let mut queue = UpdateQueue::new();
        let mut dummy1 = Dummy1 { update_count: 0, max_updates: 10 };
        let mut dummy2 = Dummy2 { has_updated: false };

        queue.push(10, &mut dummy1);
        queue.push(0, &mut dummy2);

        let mut times = [0; 13];
        let mut i = 0;

        while let Some(time) = queue.update() {
            times[i] = time;
            i += 1;
        }

        assert_eq!(times, [0, 10, 12, 14, 16, 18, 20, 22, 24, 25, 26, 28, 30]);
    }
}