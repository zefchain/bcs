// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

pub fn assert_canonical_encode_decode<T>(t: T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    let bytes = crate::to_bytes(&t).unwrap();
    let s: T = crate::from_bytes(&bytes).unwrap();
    assert_eq!(t, s);

    let mut reader = std::io::Cursor::new(bytes);
    let s_from_reader = crate::from_reader(&mut reader).unwrap();
    assert_eq!(t, s_from_reader);
}
