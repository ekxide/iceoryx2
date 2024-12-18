// Copyright (c) 2024 Contributors to the Eclipse Foundation
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

use std::time::Duration;

use examples_common::{PubSubEvent, TransmissionData};
use iceoryx2::{
    port::{listener::Listener, notifier::Notifier, subscriber::Subscriber},
    prelude::*,
    sample::Sample,
};

const HISTORY_SIZE: usize = 20;
const DEADLINE: Duration = Duration::from_secs(2);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = NodeBuilder::new().create::<ipc::Service>()?;

    let subscriber = EventBasedSubscriber::new(&node, &"My/Funk/ServiceName".try_into()?)?;

    let waitset = WaitSetBuilder::new().create::<ipc::Service>()?;
    let subscriber_guard = waitset.attach_deadline(&subscriber, DEADLINE)?;

    let on_event = |attachment_id: WaitSetAttachmentId<ipc::Service>| {
        if attachment_id.has_event_from(&subscriber_guard) {
            subscriber.handle_event().unwrap();
        } else if attachment_id.has_missed_deadline(&subscriber_guard) {
            println!(
                "Contract violation! The subscriber did not receive a message for {:?}.",
                DEADLINE
            );
        }

        CallbackProgression::Continue
    };

    waitset.wait_and_process(on_event)?;

    println!("exit");

    Ok(())
}

#[derive(Debug)]
struct EventBasedSubscriber {
    subscriber: Subscriber<ipc::Service, TransmissionData, ()>,
    notifier: Notifier<ipc::Service>,
    listener: Listener<ipc::Service>,
}

impl FileDescriptorBased for EventBasedSubscriber {
    fn file_descriptor(&self) -> &FileDescriptor {
        self.listener.file_descriptor()
    }
}

impl SynchronousMultiplexing for EventBasedSubscriber {}

impl EventBasedSubscriber {
    fn new(
        node: &Node<ipc::Service>,
        service_name: &ServiceName,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let pubsub_service = node
            .service_builder(service_name)
            .publish_subscribe::<TransmissionData>()
            .history_size(HISTORY_SIZE)
            .subscriber_max_buffer_size(HISTORY_SIZE)
            .open_or_create()?;
        let event_service = node
            .service_builder(service_name)
            .event()
            .open_or_create()?;

        let listener = event_service.listener_builder().create()?;
        let notifier = event_service.notifier_builder().create()?;
        let subscriber = pubsub_service.subscriber_builder().create()?;

        notifier.notify_with_custom_event_id(PubSubEvent::SubscriberConnected.into())?;

        Ok(Self {
            subscriber,
            listener,
            notifier,
        })
    }

    fn handle_event(&self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.listener.try_wait_one()? {
            let event: PubSubEvent = event.into();
            match event {
                PubSubEvent::SentHistory => {
                    println!("History delivered");
                    while let Ok(Some(sample)) = self.receive() {
                        println!("  history: {:?}", sample.x);
                    }
                }
                PubSubEvent::SentSample => {
                    while let Ok(Some(sample)) = self.receive() {
                        println!("received: {:?}", sample.x);
                    }
                }
                PubSubEvent::PublisherConnected => println!("new publisher connected"),
                PubSubEvent::PublisherDisconnected => println!("publisher disconnected"),
                _ => (),
            }
        }

        Ok(())
    }

    fn receive(
        &self,
    ) -> Result<Option<Sample<ipc::Service, TransmissionData, ()>>, Box<dyn std::error::Error>>
    {
        match self.subscriber.receive()? {
            Some(sample) => {
                self.notifier
                    .notify_with_custom_event_id(PubSubEvent::ReceivedSample.into())?;
                Ok(Some(sample))
            }
            None => Ok(None),
        }
    }
}

impl Drop for EventBasedSubscriber {
    fn drop(&mut self) {
        self.notifier
            .notify_with_custom_event_id(PubSubEvent::SubscriberDisconnected.into())
            .unwrap();
    }
}
