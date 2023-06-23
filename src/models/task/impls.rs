use super::{NewStep, Step, StepStatus};
use crate::{
    errors::Error,
    models::{
        goal::{Goal, Tag},
        slot::Slot,
        timeline::Timeline,
    },
};
use std::cmp::Ordering;

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.flexibility == other.flexibility
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    /// ### Custom ordering for collections of Tasks:
    ///
    /// TODO!: Rething Tags/Statusses to simplify and make this easier to understand
    ///
    /// **Careful!:** Recalculate flexibilities and re-sort after every Task/Increment placement
    /// This is required because finalizing the place(s) on the Calendar of Task/Increment makes
    /// those Slots unavailable for other Task/Increments, thus changing their flexibility. Also,
    /// some Tasks are waiting for others to be placed, and at some point they are ready to go too.
    ///
    /// 0. Exclude the following Tasks/Increments from being picked:
    /// - Scheduled
    /// - Impossible
    /// - Uninitialized (should not be there - panic if you find it!)
    /// - Blocked
    /// - BudgetMinWaitingForAdjustment
    /// - ReadyToSchedule with Remove Tag
    ///
    /// 1. Sort on Task/Increment Status first using following order:
    /// - ReadyToSchedule without Optional Tag,  without Filler Tag
    /// - ReadyToSchedule without Optional Tag, with Filler Tag
    /// - BudgetMinWaitingForAdjustment - should always be without Optional Tag
    /// - ReadyToSchedule with Optional Tag - with or without FlexDur/FlexNumber Tag
    /// - BudgetMaxWaitingForAdjustment
    ///
    ///
    /// 2. Then apply custom sort on flexibility within the Tasks/Increments with highest Status:
    /// - If there is a Tasks/Increments with flexibility 1, pick that one
    /// - If there are no more Tasks/Increments with flexibility 1 - pick the Task/Increment with **highest** flexibility
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO 2023-06-01  | Refactor for readability
        if (self.status == StepStatus::ReadyToSchedule)
            && !(other.status == StepStatus::ReadyToSchedule)
        {
            Ordering::Less
        } else if (other.status == StepStatus::ReadyToSchedule)
            && !(self.status == StepStatus::ReadyToSchedule)
        {
            Ordering::Greater
        } else if !self.tags.contains(&Tag::Optional) && other.tags.contains(&Tag::Optional) {
            Ordering::Less
        } else if self.tags.contains(&Tag::Optional) && !other.tags.contains(&Tag::Optional) {
            Ordering::Greater
        } else if !self.tags.contains(&Tag::Filler) && other.tags.contains(&Tag::Filler) {
            Ordering::Less
        } else if self.tags.contains(&Tag::Filler) && !other.tags.contains(&Tag::Filler) {
            Ordering::Greater
        } else if self.flexibility == other.flexibility {
            Ordering::Equal
        } else if self.flexibility == 1 {
            Ordering::Less
        } else if other.flexibility == 1 {
            Ordering::Greater
        } else if self.flexibility > other.flexibility {
            Ordering::Less
        } else if other.flexibility > self.flexibility {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Step {
    pub fn flexibility(&mut self) -> usize {
        self.flexibility
    }

    pub fn get_slots(&self) -> Vec<Slot> {
        self.slots.clone()
    }

    pub fn split(&mut self, counter: &mut usize) -> Result<Vec<Step>, Error> {
        // TODO 2023-06-22: Debug notes: This function not clone task.start and task.deadline
        if self.duration == 1 {
            // && !self.tags.contains(&Tag::DoNotSort) {
            return Err(Error::CannotSplit);
        }
        let mut tasks = Vec::new();
        let timeline = Timeline {
            slots: self.get_slots().into_iter().collect(),
        };
        let goal = Goal {
            id: self.goal_id.clone(),
            title: self.title.clone(),
            tags: self.tags.clone(),
            after_goals: self.after_goals.clone(),
            ..Default::default()
        };
        let new_task = NewStep {
            task_id: *counter,
            title: self.title.clone(),
            duration: 1,
            goal,
            timeline,
            status: StepStatus::Uninitialized,
            timeframe: None,
        };

        for _ in 0..self.duration {
            let mut task = Step::new(new_task.clone());
            task.id = *counter;
            task.status = StepStatus::ReadyToSchedule;
            *counter += 1;
            tasks.push(task);
        }
        Ok(tasks)
    }

    /// Remove conflicted task slots with a given slot [slot_to_remove]
    /// - Function will do nothing with Scheduled tasks
    pub fn remove_conflicted_slots(&mut self, slot_to_remove: Slot) {
        /*
        TODO 2023-06-10: Add test case to guerntee not adding extra hours for the Task.slot
        Todo: duplicate of remove_taken_slots? (NOTE: This todo need to be reviewed)
        */
        if self.status == StepStatus::Scheduled {
            return;
        }

        dbg!(&self.slots, &slot_to_remove);

        let mut slots_after_filter = Vec::new();
        for slot in &mut self.slots {
            let task_slot = *slot;
            dbg!(&slot_to_remove, &task_slot);

            let subtracted_slot = task_slot - slot_to_remove;
            dbg!(&subtracted_slot);

            slots_after_filter.extend(subtracted_slot);
            dbg!(&slots_after_filter);
        }
        dbg!(&self.slots);
        self.slots = slots_after_filter;
        dbg!(&self.slots);
        // =====

        /*
        Todo 2023-06-08:
        - create a test case and avoid using remove_taken_slots or btter approach.
        Todo 2023-06-09:
        - removed calling Task::remove_taken_slots in case TaskStatus is Blocked
        becasue it is not functional properly and need to be fixed.
        */

        self.calculate_flexibility();
    }

    pub fn remove_taken_slots(&mut self, slot_to_remove: Slot) {
        // TODO 2023-06-09  | This function is not accurate which need to be fixed and create test cases.
        let mut slots_after_filter = Vec::new();
        for task_slot in &mut self.slots {
            dbg!(&task_slot, &slot_to_remove);
            if task_slot.start >= slot_to_remove.end {
                slots_after_filter.push(*task_slot);
            }
            if task_slot.end > slot_to_remove.end && task_slot.start < slot_to_remove.start {
                task_slot.start = slot_to_remove.start;
                slots_after_filter.push(*task_slot);
            }
            if task_slot.end > slot_to_remove.end && task_slot.start >= slot_to_remove.start {
                slots_after_filter.push(*task_slot);
            }
            if !(task_slot.end <= slot_to_remove.end && task_slot.start <= slot_to_remove.start) {
                slots_after_filter.push(*task_slot);
            }

            dbg!(&slots_after_filter);
        }
        dbg!(&slots_after_filter);
        self.slots = slots_after_filter;
    }

    pub fn remove_from_blocked_by(&mut self, _id_string: String) {
        // TODO | 2023-06-06 | Seeking more info about this function
        // if self.after_goals.is_none() {
        //     return;
        // }
        // let mut ids = self.after_goals.clone().unwrap();
        // let index = ids.clone().iter().position(|x| x.eq(&id_string));
        // if index.is_some() {
        //     ids.remove(index.unwrap());
        //     if ids.is_empty() {
        //         self.after_goals = None;
        //         self.status = TaskStatus::ReadyToSchedule;
        //     } else {
        //         self.after_goals = Some(ids);
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {

    mod remove_conflicted_slots {
        use crate::models::{
            slot::Slot,
            task::{Step, StepStatus},
        };
        use chrono::Duration;

        /// Testing edge case in bug_215 which slot_to_remove
        /// is bigger than task_slot and task_slot is contained in slot_to_remove
        ///
        /// ```
        /// # "chosen_slot" to be removed from all tasks:
        /// slot_to_remove: 2023-01-03 00 to 08 (8 hours)
        /// # "task_slot" which has less duration than chosen_slot but not removed
        /// task_slot: 2023-01-03 01 to 03 (2 hour)
        /// # Also slot_to_remove contains task_slot
        ///
        /// # Output should be:
        /// task.slots.len(): 0
        /// ```
        #[test]
        fn test_intersected_bigger_slot() {
            let slot_to_remove = Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0);
            let mut task = Step::mock(
                "test",
                1,
                12,
                StepStatus::ReadyToSchedule,
                vec![Slot::mock(Duration::hours(2), 2023, 01, 03, 1, 0)],
                None,
            );

            task.remove_conflicted_slots(slot_to_remove);

            assert_eq!(task.slots.len(), 0);
        }

        #[test]
        fn test_task_is_scheduled() {
            let slot_to_remove = Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0);
            let mut task = Step::mock_scheduled(
                1,
                "1",
                "test",
                1,
                12,
                Slot::mock(Duration::hours(2), 2023, 01, 03, 1, 0),
            );

            task.remove_conflicted_slots(slot_to_remove);
            let task_after_remove = task.clone();

            assert_eq!(task_after_remove, task);
        }

        /// Test non intersected task's slots with a given slot (slot_to_remove)
        /// which returns the same task_slot
        #[test]
        fn test_nonintersected_slot() {
            let slot_to_remove = Slot::mock(Duration::hours(8), 2023, 01, 03, 0, 0);
            let task_slot = Slot::mock(Duration::hours(10), 2023, 01, 04, 1, 0);
            let mut task = Step::mock(
                "test",
                1,
                12,
                StepStatus::ReadyToSchedule,
                vec![task_slot.clone()],
                None,
            );

            task.remove_conflicted_slots(slot_to_remove);

            assert_eq!(task.slots[0], task_slot);
        }

        /// Testing normal case which removing conflicted task's slots with
        /// slot_to_remove
        /// ```
        /// slot_to_remove: 2023-01-03 00 to 03 (3 hours)
        ///
        /// task_slot: 2023-01-03 01 to 11 (10 hour)
        ///
        /// Expected:
        /// task_slot: 2023-01-03 03 to 11 (8 hour)
        /// ```
        #[test]
        fn test_normal() {
            let slot_to_remove = Slot::mock(Duration::hours(3), 2023, 01, 03, 0, 0);
            let task_slot = Slot::mock(Duration::hours(10), 2023, 01, 03, 1, 0);
            let mut task = Step::mock(
                "test",
                1,
                12,
                StepStatus::ReadyToSchedule,
                vec![task_slot.clone()],
                None,
            );

            task.remove_conflicted_slots(slot_to_remove);

            let expected_task_slot = Slot::mock(Duration::hours(8), 2023, 01, 03, 3, 0);

            assert_eq!(task.slots[0], expected_task_slot);
        }
    }

    mod split {
        use chrono::Duration;

        use crate::models::{
            slot::Slot,
            task::{Step, StepStatus},
        };

        #[test]
        fn test_split() {
            let duration: usize = 3;
            let mut counter: usize = 1;

            let goal_timeframe = Slot::mock(Duration::days(5), 2023, 6, 1, 0, 0);
            let mut task = Step::mock(
                "test",
                duration,
                0,
                StepStatus::ReadyToSchedule,
                vec![goal_timeframe],
                None,
            );
            let tasks = task.split(&mut counter).unwrap();
            dbg!(&task, &tasks);

            let mut expected_task = vec![
                Step::mock(
                    "test",
                    1,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                ),
                Step::mock(
                    "test",
                    1,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                ),
                Step::mock(
                    "test",
                    1,
                    0,
                    StepStatus::ReadyToSchedule,
                    vec![goal_timeframe],
                    None,
                ),
            ];
            expected_task[1].id = 2;
            expected_task[2].id = 3;
            dbg!(&expected_task);

            assert_eq!(tasks, expected_task);
            assert_eq!(counter, 4);

            assert_eq!(tasks[0].id, expected_task[0].id);
            assert_eq!(tasks[1].id, expected_task[1].id);
            assert_eq!(tasks[2].id, expected_task[2].id);

            assert_eq!(tasks[0].duration, expected_task[0].duration);
            assert_eq!(tasks[1].duration, expected_task[1].duration);
            assert_eq!(tasks[2].duration, expected_task[2].duration);

            assert_eq!(tasks[0].status, expected_task[0].status);
            assert_eq!(tasks[1].status, expected_task[1].status);
            assert_eq!(tasks[2].status, expected_task[2].status);
        }
    }
}
