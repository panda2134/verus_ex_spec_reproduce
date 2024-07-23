use crate::consts::*;
use crate::ext::*;

use vstd::prelude::*;

verus! {

    #[verifier::external_type_specification]
    #[verifier::external_body]
    pub struct ExDevice(Device);

    #[verifier::external_fn_specification]
    pub fn ex_read(s: &Device, out: &mut [u8; BLKSIZE]) {
        s.read(out)
    }
}
