/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub use crate::*;

extern "C" {
    pub fn PK11_FreeSlot(slot: *mut PK11SlotInfo);
    pub fn PK11_GetInternalSlot() -> *mut PK11SlotInfo;
    pub fn PK11_GenerateRandom(
        data: *mut ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    ) -> SECStatus;
    pub fn PK11_FreeSymKey(key: *mut PK11SymKey);
    pub fn PK11_ImportSymKey(
        slot: *mut PK11SlotInfo,
        type_: CK_MECHANISM_TYPE,
        origin: u32, /* PK11Origin */
        operation: CK_ATTRIBUTE_TYPE,
        key: *mut SECItem,
        wincx: *mut ::std::os::raw::c_void,
    ) -> *mut PK11SymKey;
    pub fn PK11_Derive(
        baseKey: *mut PK11SymKey,
        mechanism: CK_MECHANISM_TYPE,
        param: *mut SECItem,
        target: CK_MECHANISM_TYPE,
        operation: CK_ATTRIBUTE_TYPE,
        keySize: ::std::os::raw::c_int,
    ) -> *mut PK11SymKey;
    pub fn PK11_PubDeriveWithKDF(
        privKey: *mut SECKEYPrivateKey,
        pubKey: *mut SECKEYPublicKey,
        isSender: PRBool,
        randomA: *mut SECItem,
        randomB: *mut SECItem,
        derive: CK_MECHANISM_TYPE,
        target: CK_MECHANISM_TYPE,
        operation: CK_ATTRIBUTE_TYPE,
        keySize: ::std::os::raw::c_int,
        kdf: CK_ULONG,
        sharedData: *mut SECItem,
        wincx: *mut ::std::os::raw::c_void,
    ) -> *mut PK11SymKey;
    pub fn PK11_ExtractKeyValue(symKey: *mut PK11SymKey) -> SECStatus;
    pub fn PK11_GetKeyData(symKey: *mut PK11SymKey) -> *mut SECItem;
    pub fn PK11_GenerateKeyPair(
        slot: *mut PK11SlotInfo,
        type_: CK_MECHANISM_TYPE,
        param: *mut ::std::os::raw::c_void,
        pubk: *mut *mut SECKEYPublicKey,
        isPerm: PRBool,
        isSensitive: PRBool,
        wincx: *mut ::std::os::raw::c_void,
    ) -> *mut SECKEYPrivateKey;
    pub fn PK11_FindKeyByKeyID(
        slot: *mut PK11SlotInfo,
        keyID: *mut SECItem,
        wincx: *mut ::std::os::raw::c_void,
    ) -> *mut SECKEYPrivateKey;
    pub fn PK11_Decrypt(
        symkey: *mut PK11SymKey,
        mechanism: CK_MECHANISM_TYPE,
        param: *mut SECItem,
        out: *mut ::std::os::raw::c_uchar,
        outLen: *mut ::std::os::raw::c_uint,
        maxLen: ::std::os::raw::c_uint,
        enc: *const ::std::os::raw::c_uchar,
        encLen: ::std::os::raw::c_uint,
    ) -> SECStatus;
    pub fn PK11_Encrypt(
        symKey: *mut PK11SymKey,
        mechanism: CK_MECHANISM_TYPE,
        param: *mut SECItem,
        out: *mut ::std::os::raw::c_uchar,
        outLen: *mut ::std::os::raw::c_uint,
        maxLen: ::std::os::raw::c_uint,
        data: *const ::std::os::raw::c_uchar,
        dataLen: ::std::os::raw::c_uint,
    ) -> SECStatus;
    pub fn PK11_DestroyContext(context: *mut PK11Context, freeit: PRBool);
    pub fn PK11_CreateContextBySymKey(
        type_: CK_MECHANISM_TYPE,
        operation: CK_ATTRIBUTE_TYPE,
        symKey: *mut PK11SymKey,
        param: *mut SECItem,
    ) -> *mut PK11Context;
    pub fn PK11_DigestBegin(cx: *mut PK11Context) -> SECStatus;
    pub fn PK11_HashBuf(
        hashAlg: u32, /* SECOidTag */
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        len: PRInt32,
    ) -> SECStatus;
    pub fn PK11_DigestOp(
        context: *mut PK11Context,
        in_: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_uint,
    ) -> SECStatus;
    pub fn PK11_DigestFinal(
        context: *mut PK11Context,
        data: *mut ::std::os::raw::c_uchar,
        outLen: *mut ::std::os::raw::c_uint,
        length: ::std::os::raw::c_uint,
    ) -> SECStatus;
    pub fn PK11_DestroyGenericObject(object: *mut PK11GenericObject) -> SECStatus;
    pub fn PK11_CreateGenericObject(
        slot: *mut PK11SlotInfo,
        pTemplate: *const CK_ATTRIBUTE,
        count: ::std::os::raw::c_int,
        token: PRBool,
    ) -> *mut PK11GenericObject;
    pub fn PK11_ReadRawAttribute(
        type_: u32, /* PK11ObjectType */
        object: *mut ::std::os::raw::c_void,
        attr: CK_ATTRIBUTE_TYPE,
        item: *mut SECItem,
    ) -> SECStatus;
}
