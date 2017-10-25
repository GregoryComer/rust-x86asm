use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 139, 117, 222], OperandSize::Dword)
}

#[test]
fn vpermi2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1726538656, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 140, 117, 131, 160, 227, 232, 102], OperandSize::Dword)
}

#[test]
fn vpermi2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 133, 140, 117, 219], OperandSize::Qword)
}

#[test]
fn vpermi2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1085347594, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 131, 117, 12, 205, 10, 23, 177, 64], OperandSize::Qword)
}

#[test]
fn vpermi2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 117, 254], OperandSize::Dword)
}

#[test]
fn vpermi2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 175, 117, 36, 207], OperandSize::Dword)
}

#[test]
fn vpermi2w_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 133, 165, 117, 248], OperandSize::Qword)
}

#[test]
fn vpermi2w_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 117, 9], OperandSize::Qword)
}

#[test]
fn vpermi2w_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 203, 117, 197], OperandSize::Dword)
}

#[test]
fn vpermi2w_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ESI, 70056710, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 206, 117, 142, 6, 251, 44, 4], OperandSize::Dword)
}

#[test]
fn vpermi2w_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 181, 193, 117, 251], OperandSize::Qword)
}

#[test]
fn vpermi2w_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1023382334, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 173, 196, 117, 44, 205, 62, 147, 255, 60], OperandSize::Qword)
}

