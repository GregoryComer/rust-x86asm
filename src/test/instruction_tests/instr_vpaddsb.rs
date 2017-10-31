use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 236, 202], OperandSize::Dword)
}

#[test]
fn vpaddsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1163378356, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 236, 20, 197, 180, 190, 87, 69], OperandSize::Dword)
}

#[test]
fn vpaddsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 236, 240], OperandSize::Qword)
}

#[test]
fn vpaddsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 29686928, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 236, 4, 157, 144, 252, 196, 1], OperandSize::Qword)
}

#[test]
fn vpaddsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 236, 248], OperandSize::Dword)
}

#[test]
fn vpaddsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 1574182527, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 236, 160, 127, 30, 212, 93], OperandSize::Dword)
}

#[test]
fn vpaddsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 236, 205], OperandSize::Qword)
}

#[test]
fn vpaddsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1617695466, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 236, 20, 245, 234, 18, 108, 96], OperandSize::Qword)
}

#[test]
fn vpaddsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 141, 236, 195], OperandSize::Dword)
}

#[test]
fn vpaddsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 868646285, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 236, 140, 123, 141, 125, 198, 51], OperandSize::Dword)
}

#[test]
fn vpaddsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 142, 236, 207], OperandSize::Qword)
}

#[test]
fn vpaddsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 5, 132, 236, 12, 144], OperandSize::Qword)
}

#[test]
fn vpaddsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 169, 236, 231], OperandSize::Dword)
}

#[test]
fn vpaddsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 236, 28, 214], OperandSize::Dword)
}

#[test]
fn vpaddsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 53, 171, 236, 201], OperandSize::Qword)
}

#[test]
fn vpaddsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1878663847, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 125, 174, 236, 148, 190, 167, 34, 250, 111], OperandSize::Qword)
}

#[test]
fn vpaddsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 206, 236, 231], OperandSize::Dword)
}

#[test]
fn vpaddsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1174291835, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 206, 236, 140, 250, 123, 69, 254, 69], OperandSize::Dword)
}

#[test]
fn vpaddsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 69, 202, 236, 213], OperandSize::Qword)
}

#[test]
fn vpaddsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 2104170118, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 61, 199, 236, 180, 211, 134, 22, 107, 125], OperandSize::Qword)
}

