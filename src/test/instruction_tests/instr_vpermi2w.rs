use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 117, 251], OperandSize::Dword)
}

#[test]
fn vpermi2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 119853048, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 139, 117, 60, 77, 248, 207, 36, 7], OperandSize::Dword)
}

#[test]
fn vpermi2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 221, 141, 117, 239], OperandSize::Qword)
}

#[test]
fn vpermi2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 189, 129, 117, 3], OperandSize::Qword)
}

#[test]
fn vpermi2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 117, 211], OperandSize::Dword)
}

#[test]
fn vpermi2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 84379225, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 173, 117, 140, 151, 89, 134, 7, 5], OperandSize::Dword)
}

#[test]
fn vpermi2w_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 229, 166, 117, 255], OperandSize::Qword)
}

#[test]
fn vpermi2w_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 181, 172, 117, 1], OperandSize::Qword)
}

#[test]
fn vpermi2w_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 207, 117, 236], OperandSize::Dword)
}

#[test]
fn vpermi2w_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 206, 117, 12, 94], OperandSize::Dword)
}

#[test]
fn vpermi2w_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 149, 205, 117, 196], OperandSize::Qword)
}

#[test]
fn vpermi2w_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 622592179, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 141, 195, 117, 60, 133, 179, 0, 28, 37], OperandSize::Qword)
}

