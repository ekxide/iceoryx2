// Copyright (c) 2025 Contributors to the Eclipse Foundation
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

use iceoryx2::service::builder::CustomKeyMarker;
use pyo3::prelude::*;

use crate::error::ReaderCreateError;
use crate::parc::Parc;
use crate::port_factory_blackboard::PortFactoryBlackboardType;
use crate::reader::{Reader, ReaderType};
use crate::type_storage::TypeStorage;

type IpcPortFactoryReader<'a> = iceoryx2::service::port_factory::reader::PortFactoryReader<
    'a,
    crate::IpcService,
    CustomKeyMarker,
>;
type LocalPortFactoryReader<'a> = iceoryx2::service::port_factory::reader::PortFactoryReader<
    'a,
    crate::LocalService,
    CustomKeyMarker,
>;

pub(crate) enum PortFactoryReaderType {
    Ipc(Parc<IpcPortFactoryReader<'static>>),
    Local(Parc<LocalPortFactoryReader<'static>>),
}

#[pyclass]
/// Factory to create a new `Reader` port/endpoint for `MessagingPattern::Blackboard`
/// based communication.
pub struct PortFactoryReader {
    factory: Parc<PortFactoryBlackboardType>,
    value: PortFactoryReaderType,
    key_type_details: TypeStorage,
}

impl PortFactoryReader {
    pub(crate) fn new(
        factory: Parc<PortFactoryBlackboardType>,
        key_type_details: TypeStorage,
    ) -> Self {
        Self {
            factory: factory.clone(),
            value: match &*factory.lock() {
                PortFactoryBlackboardType::Ipc(v) => PortFactoryReaderType::Ipc(unsafe {
                    Parc::new(core::mem::transmute::<
                        IpcPortFactoryReader<'_>,
                        IpcPortFactoryReader<'static>,
                    >(v.reader_builder()))
                }),
                PortFactoryBlackboardType::Local(v) => PortFactoryReaderType::Local(unsafe {
                    Parc::new(core::mem::transmute::<
                        LocalPortFactoryReader<'_>,
                        LocalPortFactoryReader<'static>,
                    >(v.reader_builder()))
                }),
            },
            key_type_details,
        }
    }

    fn clone_ipc(&self, value: IpcPortFactoryReader<'static>) -> Self {
        Self {
            factory: self.factory.clone(),
            value: PortFactoryReaderType::Ipc(Parc::new(value)),
            key_type_details: self.key_type_details.clone(),
        }
    }

    fn clone_local(&self, value: LocalPortFactoryReader<'static>) -> Self {
        Self {
            factory: self.factory.clone(),
            value: PortFactoryReaderType::Local(Parc::new(value)),
            key_type_details: self.key_type_details.clone(),
        }
    }
}

#[pymethods]
impl PortFactoryReader {
    /// Creates a new `Reader` or emits a `ReaderCreateError` on failure.
    pub fn create(&self) -> PyResult<Reader> {
        let _guard = self.factory.lock();
        match &self.value {
            PortFactoryReaderType::Ipc(v) => {
                let this = (*v.lock()).clone();
                Ok(Reader {
                    value: Parc::new(ReaderType::Ipc(Some(
                        this.create()
                            .map_err(|e| ReaderCreateError::new_err(format!("{e:?}")))?,
                    ))),
                    key_type_details: self.key_type_details.clone(),
                })
            }
            PortFactoryReaderType::Local(v) => {
                let this = (*v.lock()).clone();
                Ok(Reader {
                    value: Parc::new(ReaderType::Local(Some(
                        this.create()
                            .map_err(|e| ReaderCreateError::new_err(format!("{e:?}")))?,
                    ))),
                    key_type_details: self.key_type_details.clone(),
                })
            }
        }
    }
}
