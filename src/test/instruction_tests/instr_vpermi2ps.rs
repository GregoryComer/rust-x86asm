use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 119, 246], OperandSize::Dword)
}

#[test]
fn vpermi2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 1688876121, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 140, 119, 134, 89, 52, 170, 100], OperandSize::Dword)
}

#[test]
fn vpermi2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 75067327, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 153, 119, 190, 191, 111, 121, 4], OperandSize::Dword)
}

#[test]
fn vpermi2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 21, 141, 119, 208], OperandSize::Qword)
}

#[test]
fn vpermi2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 37, 134, 119, 44, 247], OperandSize::Qword)
}

#[test]
fn vpermi2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RAX, 1965882304, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 146, 119, 144, 192, 251, 44, 117], OperandSize::Qword)
}

#[test]
fn vpermi2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 119, 215], OperandSize::Dword)
}

#[test]
fn vpermi2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 1534199722, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 119, 175, 170, 7, 114, 91], OperandSize::Dword)
}

#[test]
fn vpermi2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 189, 119, 36, 199], OperandSize::Dword)
}

#[test]
fn vpermi2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 5, 171, 119, 244], OperandSize::Qword)
}

#[test]
fn vpermi2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 7718069, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 93, 175, 119, 132, 251, 181, 196, 117, 0], OperandSize::Qword)
}

#[test]
fn vpermi2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2121267419, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 37, 179, 119, 36, 93, 219, 248, 111, 126], OperandSize::Qword)
}

#[test]
fn vpermi2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 202, 119, 254], OperandSize::Dword)
}

#[test]
fn vpermi2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 367418501, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 203, 119, 148, 222, 133, 92, 230, 21], OperandSize::Dword)
}

#[test]
fn vpermi2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ESI, 1634524551, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 218, 119, 166, 135, 221, 108, 97], OperandSize::Dword)
}

#[test]
fn vpermi2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 93, 199, 119, 206], OperandSize::Qword)
}

#[test]
fn vpermi2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1687000324, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 205, 119, 12, 189, 4, 149, 141, 100], OperandSize::Qword)
}

#[test]
fn vpermi2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RDX, 463687987, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 37, 215, 119, 178, 51, 81, 163, 27], OperandSize::Qword)
}

