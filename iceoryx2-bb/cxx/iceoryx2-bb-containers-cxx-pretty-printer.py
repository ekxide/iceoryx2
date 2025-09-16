# Copyright (c) 2025 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache Software License 2.0 which is available at
# https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
# which is available at https://opensource.org/licenses/MIT.
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

import gdb
import re

class OptionalPrinter:
    "Print an iox2::container::Optional"

    def __init__(self, val, *, contained_type):
        self.val = val
        self.contained_type = contained_type

    def to_string(self):
        is_empty = self.val['m_value']['m_is_empty']
        if is_empty:
            return f"{{ empty Optional<{self.contained_type}> }}"
        else:
            # the type of the nested value is determined as remove_cv_ref_t,
            # which is not pretty-printed. we need to jump through a few hoops
            # to get the correct nested type to enable pretty printing of the
            # contained value
            nested_type = self.val['m_value']['m_u_value'].type.strip_typedefs()
            return f"{{value = {self.val['m_value']['m_u_value'].cast(nested_type)}}}"

def iox2_bb_containers_cxx(val):
    iox2_bb_containers_cxx.rx_optional = re.compile("^(?:const )?iox2::container::Optional<(.*)>$")
    if (match := iox2_bb_containers_cxx.rx_optional.match(str(val.type))) is not None:
        return OptionalPrinter(val, contained_type=match[1])
    else:
        return None

gdb.pretty_printers.append(iox2_bb_containers_cxx)
