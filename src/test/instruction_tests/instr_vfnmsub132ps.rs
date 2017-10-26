use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 158, 223], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 828014092, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 158, 28, 85, 12, 126, 90, 49], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 158, 200], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 146685203, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 158, 52, 157, 19, 61, 190, 8], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 158, 195], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 158, 26], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 158, 204], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 158, 17], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 158, 240], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 158, 12, 121], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 2026518036, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 156, 158, 148, 136, 20, 54, 202, 120], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 85, 131, 158, 209], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 101, 137, 158, 52, 222], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM11)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 37, 159, 158, 41], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 171, 158, 227], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1582886960, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 158, 36, 213, 48, 240, 88, 94], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 2107130702, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 186, 158, 12, 77, 78, 67, 152, 125], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 5, 170, 158, 234], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 5, 166, 158, 52, 86], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1530750718, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 180, 158, 180, 190, 254, 102, 61, 91], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 158, 158, 240], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 206, 158, 57], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1741189974, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 221, 158, 44, 253, 86, 115, 200, 103], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 117, 244, 158, 242], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 5, 196, 158, 47], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 117, 211, 158, 20, 115], OperandSize::Qword)
}

