use crate::repetition::Repetition;
use crate::slot_generator::slot_generator;
use crate::task::Task;
use crate::time_slot_iterator::TimeSlotIterator;
use chrono::{Duration, NaiveDateTime, Timelike};
use serde::Deserialize;
use std::option::Option;

#[derive(Deserialize, Debug, Default)]
pub struct Goal {
    pub id: String,
    pub title: String,
    /// How much total time should a user put into their goal, eg "I want to learn how to code, and I want to code 6 hours per day"
    pub duration: usize,
    pub repeat: Option<Repetition>,
    /// start date bound for this Goal's Tasks
    #[serde(default)]
    pub start: Option<NaiveDateTime>,
    /// deadline date bound for this Goal's Tasks
    #[serde(default)]
    pub deadline: Option<NaiveDateTime>,
    /// start time bound after which activity should be done
    #[serde(default)]
    pub after_time: Option<usize>,
    /// deadline time bound before which activity should be done
    #[serde(default)]
    pub before_time: Option<usize>,
}

//#[cfg(test)]
impl Goal {
    pub fn new(id: usize) -> Self {
        Self {
            id: id.to_string(),
            title: String::from("Test"),
            ..Default::default()
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn duration(mut self, duration: usize) -> Self {
        self.duration = duration;
        self
    }

    pub fn repeat(mut self, repetition: Repetition) -> Self {
        self.repeat = Some(repetition);
        self
    }

    pub fn start(mut self, start: NaiveDateTime) -> Self {
        self.start = Some(start);
        self
    }

    pub fn deadline(mut self, deadline: NaiveDateTime) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn after_time(mut self, after_time: usize) -> Self {
        self.after_time = Some(after_time);
        self
    }

    pub fn before_time(mut self, before_time: usize) -> Self {
        self.before_time = Some(before_time);
        self
    }

    pub fn generate_tasks(
        self,
        calendar_start: NaiveDateTime,
        calendar_end: NaiveDateTime,
        counter: &mut usize,
    ) -> Vec<Task> {
        let mut tasks = Vec::new();
        /*If the repetition of the goal is DAILY, a different task will be generated for each day between
         **the start and deadline.
         **If the repetition is MONDAYS, a different task will be generated for each monday
         **between the start and deadline.
         **If the repetition is WEEKLY, a different task will be generated for each mon-sun
         **period between the start and deadline. etc...(to see all handled scenarios see time_slot_iterator.rs.)
         **.
         **.
         **.
         **If the repetition is NONE, only one task will be generated for the period between
         **the start and deadline.*/
        let mut start = self.start.unwrap_or(calendar_start);
        let deadline = self.deadline.unwrap_or(calendar_end);
        if let Some(Repetition::EveryXhours(_)) = self.repeat {
            start += Duration::hours(self.after_time.unwrap_or(0) as i64);
        }
        let time_periods = TimeSlotIterator {
            start,
            end: deadline,
            repetition: self.repeat,
        };
        let tasks_per_period = match self.repeat {
            Some(Repetition::WEEKLY(x)) => x,
            Some(Repetition::DAILY(x)) => x,
            _ => 1,
        };
        for time_period in time_periods {
            for _ in 0..tasks_per_period {
                let task_id = *counter;
                *counter += 1;
                //assign slots that are within the specified after_time and before_time
                let slots =
                    slot_generator(self.after_time, self.before_time, &time_period, self.repeat);
                //the following 'if' check is needed because of every-x-hours repetitions.
                //the slot_generator doesn't currently handle 'every-x-hours' repetitions very well.
                //after the slots have been assigned, we need to ignore slots that are not within
                //the after/before time of the goal.
                //this may be improved upon by a wider refactor of the slot_generator.
                if self.before_time.unwrap_or(24) < self.after_time.unwrap_or(0) {
                    if slots.iter().any(|slot| {
                        (slot.start.hour() as usize) > self.before_time.unwrap_or(24)
                            && (slot.start.hour() as usize) < self.after_time.unwrap_or(0)
                    }) {
                        continue;
                    }
                } else if slots.iter().any(|slot| {
                    (slot.start.hour() as usize) >= self.before_time.unwrap_or(24)
                        || (slot.start.hour() as usize) < self.after_time.unwrap_or(0)
                }) {
                    continue;
                }

                //calculate flexibility
                let mut flexibility = 0;
                for slot in &slots {
                    flexibility += slot.num_hours() - self.duration + 1;
                }
                let mut a_time = self.after_time.unwrap_or(0);
                let mut b_time = self.before_time.unwrap_or(24);
                //handle edge case of everyxhours (the tasks should have after and before time of the time period, not of the goal)
                if let Some(Repetition::EveryXhours(_)) = self.repeat {
                    a_time = time_period.start.hour() as usize;
                    b_time = time_period.end.hour() as usize;
                }
                let t = Task::new(
                    task_id,
                    start,
                    deadline,
                    slots,
                    flexibility,
                    &self,
                    a_time,
                    b_time,
                );
                tasks.push(t);
            }
        }
        tasks
    }
}
