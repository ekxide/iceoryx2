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

#include "iox2/bb/static_string.hpp"
#include "iox2/iceoryx2.hpp"
#include "parse_args.hpp"
#include "transmission_data.hpp"

#include <iostream>

constexpr iox2::bb::Duration CYCLE_TIME = iox2::bb::Duration::from_millis(1);

auto main(int argc, char** argv) -> int {
    using namespace iox2;
    std::cout << "#### publisher starting" << std::endl;
    set_log_level_from_env_or(LogLevel::Info);

    check_for_help_from_args(argc, argv, []() -> auto {
        std::cout << "Publisher of the domain example." << std::endl;
        std::cout << std::endl;
        std::cout << "Use '-d' or '--domain' to specify the name of the domain." << std::endl;
        std::cout << "Use '-s' or '--service' to specify the name of the service." << std::endl;
    });

    std::cout << "#### publisher 1000" << std::endl;

    // NOLINTNEXTLINE(cppcoreguidelines-avoid-magic-numbers,readability-magic-numbers) fine for the example
    const CliOption<32> option_domain { "-d",
                                        "--domain",
                                        iox2::bb::StaticString<32>::from_utf8_unchecked("iox2_"),
                                        "Invalid parameter! The domain must be passed after '-d' or '--domain'" };
    // NOLINTNEXTLINE(cppcoreguidelines-avoid-magic-numbers,readability-magic-numbers) fine for the example
    const CliOption<256> option_service { "-s",
                                          "--service",
                                          iox2::bb::StaticString<256>::from_utf8_unchecked("my_funky_service"),
                                          "Invalid parameter! The service must be passed after '-s' or '--service'" };

    auto domain = parse_from_args(argc, argv, option_domain);
    auto service_name = parse_from_args(argc, argv, option_service);

    std::cout << "#### publisher 2000" << std::endl;

    // create a new config based on the global config
    auto config = Config::global_config().to_owned();

    // The domain name becomes the prefix for all resources.
    // Therefore, different domain names never share the same resources.
    config.global().set_prefix(iox2::bb::FileName::create(domain).value());

    std::cout << "#### publisher 3000" << std::endl;

    auto node = NodeBuilder()
                    // use the custom config when creating the custom node
                    // every service constructed by the node will use this config
                    .config(config)
                    .create<ServiceType::Ipc>()
                    .value();

    auto service = node.service_builder(ServiceName::create(service_name.unchecked_access().c_str()).value())
                       .publish_subscribe<TransmissionData>()
                       .open_or_create()
                       .value();

    std::cout << "#### publisher 4000" << std::endl;

    auto publisher = service.publisher_builder().create().value();

    auto counter = 0;
    auto node_wait_result = node.wait(CYCLE_TIME);
    while (node_wait_result.has_value()) {
        counter += 1;

        auto sample = publisher.loan_uninit().value();

        auto initialized_sample =
            sample.write_payload(TransmissionData { counter, counter * 3, counter * 812.12 }); // NOLINT

        send(std::move(initialized_sample)).value();

        std::cout << "[domain: \"" << domain.unchecked_access().c_str() << "\", service: \""
                  << service_name.unchecked_access().c_str() << "] Send sample " << counter << "..." << std::endl;

        node_wait_result = node.wait(CYCLE_TIME);
    }

    if (!node_wait_result.has_value()) {
        std::cout << "#### node_wait_result has error: " << static_cast<uint32_t>(node_wait_result.error())
                  << std::endl;
    }

    std::cout << "exit" << std::endl;

    return 0;
}
