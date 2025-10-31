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

use std::sync::Arc;

use iceoryx2::service::builder::CustomKeyMarker;
use iceoryx2_bb_log::fatal_panic;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::entry_handle::{EntryHandle, EntryHandleType};
use crate::error::EntryHandleError;
use crate::parc::Parc;
use crate::type_detail::TypeDetail;
use crate::type_storage::TypeStorage;
use crate::unique_reader_id::UniqueReaderId;

pub(crate) enum ReaderType {
    Ipc(Option<iceoryx2::port::reader::Reader<crate::IpcService, CustomKeyMarker>>),
    Local(Option<iceoryx2::port::reader::Reader<crate::LocalService, CustomKeyMarker>>),
}

#[pyclass]
/// Represents the reading endpoint of a blackboard based communication.
pub struct Reader {
    pub(crate) value: Parc<ReaderType>,
    pub(crate) key_type_details: TypeStorage,
}

#[pymethods]
impl Reader {
    #[getter]
    pub fn __key_type_details(&self) -> Option<Py<PyAny>> {
        self.key_type_details.clone().value
    }

    #[getter]
    /// Returns the `UniqueReaderId` of the `Reader`
    pub fn id(&self) -> UniqueReaderId {
        match &*self.value.lock() {
            ReaderType::Ipc(Some(v)) => UniqueReaderId(v.id()),
            ReaderType::Local(Some(v)) => UniqueReaderId(v.id()),
            _ => fatal_panic!(from "Reader::id()",
                    "Accessing a deleted reader."),
        }
    }

    /// Creates an EntryHandle for direct read access to the value. On failure
    /// it returns `EntryHandleError` describing the failure.
    pub fn __entry(&self, key: PyObject, value_type_details: &TypeDetail) -> PyResult<EntryHandle> {
        Python::with_gil(|py| {
            let key = key.downcast_bound::<PyBytes>(py).unwrap(); // TODO: error handling
            let key = key.as_bytes();

            match &*self.value.lock() {
                ReaderType::Ipc(Some(v)) => {
                    let entry_handle = unsafe {
                        v.__internal_entry(key.as_ptr(), &value_type_details.0)
                            .map_err(|e| EntryHandleError::new_err(format!("{e:?}")))?
                    };
                    Ok(EntryHandle {
                        value: EntryHandleType::Ipc(Some(entry_handle)),
                    })
                }
                ReaderType::Local(Some(v)) => {
                    let entry_handle = unsafe {
                        v.__internal_entry(key.as_ptr(), &value_type_details.0)
                            .map_err(|e| EntryHandleError::new_err(format!("{e:?}")))?
                    };
                    Ok(EntryHandle {
                        value: EntryHandleType::Local(Some(entry_handle)),
                    })
                }
                _ => fatal_panic!(from "Reader::entry()",
                    "Accessing a deleted reader."),
            }
        })
    }

    /// Releases the `Reader`.
    ///
    /// After this call the `Reader` is no longer usable!
    pub fn delete(&mut self) {
        match *self.value.lock() {
            ReaderType::Ipc(ref mut v) => {
                v.take();
            }
            ReaderType::Local(ref mut v) => {
                v.take();
            }
        }
    }
}
