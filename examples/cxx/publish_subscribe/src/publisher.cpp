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

#include "iox2/iceoryx2.hpp"
#include "transmission_data.hpp"

#include <iostream>
// #include <utility>

// constexpr iox2::bb::Duration CYCLE_TIME = iox2::bb::Duration::from_secs(1);

auto main() -> int {
    using namespace iox2;
    set_log_level_from_env_or(LogLevel::Info);

    auto config = Config::global_config().to_owned();
    config.global().node().set_cleanup_dead_nodes_on_creation(true);
    config.global().node().set_cleanup_dead_nodes_on_destruction(true);
    config.global().service().set_cleanup_dead_nodes_on_open(true);

    auto list_nodes = [&] {
        Node<ServiceType::Ipc>::list(config.view(), [](auto node_state) {
            node_state.alive([](const AliveNodeView<ServiceType::Ipc>& view) {
                if (view.details().has_value()) {
                    std::cout << "  alive: " << view.details()->executable().as_string() << std::endl;
                } else {
                    std::cout << "  alive: " << view.id() << std::endl;
                }
            });
            node_state.dead([](const DeadNodeView<ServiceType::Ipc>& view) {
                if (view.details().has_value()) {
                    std::cout << "  dead: " << view.details()->executable().as_string() << std::endl;
                } else {
                    std::cout << "  dead: " << view.id() << std::endl;
                }
            });
            node_state.inaccessible(
                [](const UniqueNodeId& view) { std::cout << "  inaccessible: " << view << std::endl; });
            node_state.undefined([](const UniqueNodeId& view) { std::cout << "  undefined: " << view << std::endl; });


            return CallbackProgression::Continue;
        });
    };

    {
        std::cout << "before cleanup via node creation" << std::endl;
        list_nodes();

        auto node = NodeBuilder().config(config).create<ServiceType::Ipc>().value();

        std::cout << "before cleanup via service open" << std::endl;
        list_nodes();

        node.service_builder(ServiceName::create("My/Funk/ServiceName").value())
            .request_response<uint64_t, TransmissionData>()
            .max_servers(2)
            .max_clients(1)
            .open_or_create();
    }

    std::cout << "after cleanup" << std::endl;
    list_nodes();

    // auto service = node.service_builder(ServiceName::create("My/Funk/ServiceName").value())
    //                    .publish_subscribe<TransmissionData>()
    //                    .open_or_create()
    //                    .value();
    //
    // auto publisher = service.publisher_builder().create().value();
    //
    // auto counter = 0;
    // while (node.wait(CYCLE_TIME).has_value()) {
    //     counter += 1;
    //
    //     auto sample = publisher.loan_uninit().value();
    //
    //     auto initialized_sample =
    //         sample.write_payload(TransmissionData { counter, counter * 3, counter * 812.12 }); // NOLINT
    //
    //     send(std::move(initialized_sample)).has_value();
    //
    //     std::cout << "Send sample " << counter << "..." << std::endl;
    // }
    //
    // std::cout << "exit" << std::endl;

    return 0;
}
