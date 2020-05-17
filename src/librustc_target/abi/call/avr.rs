// Reference: avr-gcc Wiki
// https://gcc.gnu.org/wiki/avr-gcc

use crate::abi::call::{ArgAbi, FnAbi};

// avr-gcc Wiki § Calling Convention
//
// "Return values with a size of 1 byte up to and including a size of 8
// bytes will be returned in registers. Return values whose size is outside
// that range will be returned in memory."
//
//
// "If a return value cannot be returned in registers, the caller will
// allocate stack space and pass the address as implicit first pointer
// argument to the callee. The callee will put the return value into
// the space provided by the caller."
fn classify_ret_ty<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    if ret.layout.is_aggregate() {
        ret.make_indirect();
    } else {
        ret.extend_integer_width_to(8); // Is 8 correct?
    }
}

// avr-gcc Wiki § Calling Convention
//
// "An argument is passed either completely in registers or completely in memory."
//
// "Arguments of varargs functions are passed on the stack. This applies
// even to the named arguments."
fn classify_arg_ty<Ty>(arg: &mut ArgAbi<'_, Ty>) {
    if arg.layout.is_aggregate() {
        arg.make_indirect();
    } else {
        arg.extend_integer_width_to(8);
    }
}

pub fn compute_abi_info<Ty>(fty: &mut FnAbi<'_, Ty>) {
    if !fty.ret.is_ignore() {
        classify_ret_ty(&mut fty.ret);
    }

    for arg in &mut fty.args {
        if arg.is_ignore() {
            continue;
        }

        classify_arg_ty(arg);
    }
}
