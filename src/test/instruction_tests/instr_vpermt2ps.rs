use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 139, 127, 197], OperandSize::Dword)
}

#[test]
fn vpermt2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 127, 52, 219], OperandSize::Dword)
}

#[test]
fn vpermt2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1597771602, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 158, 127, 60, 221, 82, 15, 60, 95], OperandSize::Dword)
}

#[test]
fn vpermt2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 21, 135, 127, 193], OperandSize::Qword)
}

#[test]
fn vpermt2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 781520787, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 85, 143, 127, 52, 125, 147, 15, 149, 46], OperandSize::Qword)
}

#[test]
fn vpermt2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 45, 149, 127, 44, 81], OperandSize::Qword)
}

#[test]
fn vpermt2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 127, 228], OperandSize::Dword)
}

#[test]
fn vpermt2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 127, 60, 207], OperandSize::Dword)
}

#[test]
fn vpermt2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 127, 44, 223], OperandSize::Dword)
}

#[test]
fn vpermt2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 77, 171, 127, 244], OperandSize::Qword)
}

#[test]
fn vpermt2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 172, 127, 51], OperandSize::Qword)
}

#[test]
fn vpermt2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1795367149, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 21, 188, 127, 60, 141, 237, 32, 3, 107], OperandSize::Qword)
}

#[test]
fn vpermt2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 207, 127, 230], OperandSize::Dword)
}

#[test]
fn vpermt2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 205, 127, 58], OperandSize::Dword)
}

#[test]
fn vpermt2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 221, 127, 40], OperandSize::Dword)
}

#[test]
fn vpermt2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 194, 127, 252], OperandSize::Qword)
}

#[test]
fn vpermt2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 77, 194, 127, 63], OperandSize::Qword)
}

#[test]
fn vpermt2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1805667232, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 37, 214, 127, 36, 117, 160, 75, 160, 107], OperandSize::Qword)
}

