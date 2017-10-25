use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 248, 193], OperandSize::Dword)
}

#[test]
fn vpsubb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 2045943881, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 248, 20, 149, 73, 160, 242, 121], OperandSize::Dword)
}

#[test]
fn vpsubb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 248, 231], OperandSize::Qword)
}

#[test]
fn vpsubb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1616554718, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 248, 52, 77, 222, 170, 90, 96], OperandSize::Qword)
}

#[test]
fn vpsubb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 248, 198], OperandSize::Dword)
}

#[test]
fn vpsubb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 353110148, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 248, 156, 66, 132, 8, 12, 21], OperandSize::Dword)
}

#[test]
fn vpsubb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 248, 217], OperandSize::Qword)
}

#[test]
fn vpsubb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 248, 1], OperandSize::Qword)
}

#[test]
fn vpsubb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 248, 240], OperandSize::Dword)
}

#[test]
fn vpsubb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 143, 248, 44, 211], OperandSize::Dword)
}

#[test]
fn vpsubb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 45, 129, 248, 199], OperandSize::Qword)
}

#[test]
fn vpsubb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 707010252, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 5, 135, 248, 44, 221, 204, 30, 36, 42], OperandSize::Qword)
}

#[test]
fn vpsubb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 170, 248, 251], OperandSize::Dword)
}

#[test]
fn vpsubb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 169, 248, 36, 143], OperandSize::Dword)
}

#[test]
fn vpsubb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 61, 164, 248, 240], OperandSize::Qword)
}

#[test]
fn vpsubb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1737780384, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 170, 248, 52, 117, 160, 108, 148, 103], OperandSize::Qword)
}

#[test]
fn vpsubb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 207, 248, 193], OperandSize::Dword)
}

#[test]
fn vpsubb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 248, 34], OperandSize::Dword)
}

#[test]
fn vpsubb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 69, 203, 248, 236], OperandSize::Qword)
}

#[test]
fn vpsubb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 37, 201, 248, 4, 78], OperandSize::Qword)
}

