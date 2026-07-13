// Copyright (c) 2026 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use iceoryx2_bb_container::queue::RelocatableContainer;
use iceoryx2_bb_container::vector::*;
use iceoryx2_bb_derive_macros::ZeroCopySend;
use iceoryx2_bb_elementary_traits::zero_copy_send::ZeroCopySend;
pub use iceoryx2_bb_lock_free::mpmc::counting_bit_set::RelocatableCountingBitSet;
use iceoryx2_log::fail;

use crate::event::{
    EventId,
    event_state::{
        EventActivation, EventState, EventStateActivateError, GroupId, GroupInfo,
        GroupInfoContainer,
    },
};

#[derive(Debug, ZeroCopySend)]
#[repr(C)]
pub struct RelocatableCountingBitSetEventState {
    bitset: RelocatableCountingBitSet,
    group_info: GroupInfoContainer,
}

impl EventState for RelocatableCountingBitSetEventState {
    fn set_event_groups(&mut self, grouped_events: &[(GroupId, EventId)]) {
        if let Some((group_id, event_id)) = grouped_events.first() {
            self.group_info[event_id.as_value()].id = group_id.as_value()
        }

        let mut iter = grouped_events.windows(2);
        while let Some(
            &[
                (prev_group_id, previous_event_id),
                (current_group_id, current_event_id),
            ],
        ) = iter.next()
        {
            self.group_info[current_event_id.as_value()].id = current_group_id.as_value();

            if current_group_id.as_value() == prev_group_id.as_value() {
                self.group_info[previous_event_id.as_value()].next_group_item_index =
                    current_event_id.as_value();
                self.group_info[current_event_id.as_value()].previous_group_item_index =
                    previous_event_id.as_value();
            }
        }
    }

    fn max_event_count(&self) -> u64 {
        RelocatableCountingBitSet::max_count()
    }

    fn max_event_id(&self) -> EventId {
        EventId::new(self.bitset.capacity().saturating_sub(1))
    }

    fn activate(&self, event_id: EventId) -> Result<bool, EventStateActivateError> {
        if self.max_event_id() < event_id {
            fail!(from self, with EventStateActivateError::EventIdOutOfBounds,
                "Unable to activate {event_id:?} since it is out of bounds (max = {:?}).", self.max_event_id());
        }

        self.bitset.set(event_id.as_value());

        // check if there are events in the same group before the index of the event id currently set
        let mut current_group_item_index = event_id.as_value();
        loop {
            let previous_group_item_index =
                self.group_info[current_group_item_index].previous_group_item_index;
            if previous_group_item_index == current_group_item_index {
                break;
            }
            if self.bitset.peak(previous_group_item_index) == 0 {
                return Ok(false);
            }
            current_group_item_index = previous_group_item_index;
        }

        // check if there are events in the same group after the index of the event id currently set
        let mut current_group_item_index = event_id.as_value();
        loop {
            let next_group_item_index =
                self.group_info[current_group_item_index].next_group_item_index;
            if next_group_item_index == current_group_item_index {
                break;
            }
            if self.bitset.peak(next_group_item_index) == 0 {
                return Ok(false);
            }
            current_group_item_index = next_group_item_index;
        }

        // it seems all event ids from the group are set
        Ok(true)
    }

    fn drain<F: FnMut(EventActivation)>(&self, callback: &mut F) -> u64 {
        let mut counter = 0;
        for (i, group_info) in self.group_info.iter().enumerate() {
            let mut reset_bit = |id| {
                self.bitset.reset(id, |bit_state| {
                    counter += bit_state.count();
                    callback(EventActivation {
                        id: EventId::new(bit_state.bit()),
                        count: bit_state.count(),
                    });
                });
            };

            if group_info.id == GroupId::NO_GROUP {
                reset_bit(i);
            } else
            // first event id in the group
            if group_info.previous_group_item_index == i {
                let mut all_bits_in_group_set = true;
                let mut current_group_item_index = i;
                // at first, check if all events in the group are set
                loop {
                    all_bits_in_group_set &= self.bitset.peak(current_group_item_index) != 0;

                    let next_group_item_index =
                        self.group_info[current_group_item_index].next_group_item_index;
                    if next_group_item_index == current_group_item_index {
                        break;
                    }

                    current_group_item_index = next_group_item_index;
                }

                if !all_bits_in_group_set {
                    continue;
                }

                let mut current_group_item_index = i;
                // now reset all bits from the group
                loop {
                    reset_bit(current_group_item_index);

                    let next_group_item_index =
                        self.group_info[current_group_item_index].next_group_item_index;
                    if next_group_item_index == current_group_item_index {
                        break;
                    }

                    current_group_item_index = next_group_item_index;
                }
            }
        }

        counter
    }
}

impl RelocatableContainer for RelocatableCountingBitSetEventState {
    unsafe fn new_uninit(capacity: usize) -> Self {
        unsafe {
            Self {
                bitset: RelocatableCountingBitSet::new_uninit(capacity),
                group_info: GroupInfoContainer::new_uninit(capacity),
            }
        }
    }

    unsafe fn init<T: iceoryx2_bb_elementary::bump_allocator::BaseAllocator>(
        &mut self,
        allocator: &T,
    ) -> Result<(), iceoryx2_bb_elementary::bump_allocator::AllocationError> {
        unsafe {
            self.bitset.init(allocator)?;
            self.group_info.init(allocator)?;
        }

        for i in 0..self.group_info.capacity() {
            unsafe {
                self.group_info.push_unchecked(GroupInfo {
                    id: GroupId::NO_GROUP,
                    next_group_item_index: i,
                    previous_group_item_index: i,
                });
            }
        }

        Ok(())
    }

    fn memory_size(capacity: usize) -> usize {
        RelocatableCountingBitSet::const_memory_size(capacity)
            + GroupInfoContainer::const_memory_size(capacity)
    }
}
