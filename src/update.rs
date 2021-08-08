use crate::Message;

use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt,
    fmt::{Debug, Formatter}
};

/// Any entity that needs to be processed in the update queue needs to implement the Updatable trait
pub trait Updatable {
    /// Update the entity.
    ///
    /// The return value is a tuple of two parts. The first is an `Option(u32)` that contains either
    /// the number of ticks until the next scheduled update for this entity, or `None` if it should
    /// not be update again.
    ///
    /// The second part of the return value is a vector of `Effect`s. An effect represents any outcome
    /// of this update that affects systems outside of the update queue itself. For example, any messages
    /// that should be logged for the player to see as a result of this update would be included in
    /// the effects vector. This allows the update queue to be decoupled from other systems of the
    /// game (some of which, such as a renderer, aren't even part of `yanor_core`).
    ///
    /// It is (almost?) never necessary to call this directly; instead it should be called only via
    /// UpdateQueue.update, which will always check whether an updatable is dead before updating.
    fn update(&mut self, effects: &mut Vec<Effect>) -> Option<u32>;

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
    /// should not normally happen for any reason other than the end of a game.
    ///
    /// The heat death of the universe occurs when the update queue is empty.
    pub fn update(&mut self, effects: &mut Vec<Effect>) -> Option<u32> {
        let mut info = self.queue.pop()?;

        while !info.updatable.is_active() {
            info = self.queue.pop()?;
        }

        let now = info.time;

        if let Some(dt) = info.updatable.update(effects) {
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

/// An `Effect` represents any side effect of an update that affects systems outside of the update
/// queue and the entity being updated itself. This makes other game systems less coupled with the
/// update system, and also gives interfaces to the game leeway to interpret effects in different ways.
pub enum Effect {
    Log(Message)
}