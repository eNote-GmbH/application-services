/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub type PRIntn = ::std::os::raw::c_int;
pub type PRBool = PRIntn;
pub type PRUword = ::std::os::raw::c_ulong;
pub type PRInt32 = ::std::os::raw::c_int;
pub type PRUint32 = ::std::os::raw::c_uint;
pub const PR_FALSE: PRBool = 0;
pub const PR_TRUE: PRBool = 1;
