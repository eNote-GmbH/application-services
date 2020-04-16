/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::*;

extern "C" {
    pub fn PORT_FreeArena(arena: *mut PLArenaPool, zero: PRBool);
    pub fn NSS_SecureMemcmp(
        a: *const ::std::os::raw::c_void,
        b: *const ::std::os::raw::c_void,
        n: size_t,
    ) -> ::std::os::raw::c_int;
}

pub type size_t = ::std::os::raw::c_ulong;
