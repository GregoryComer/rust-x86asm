use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 125, 203], OperandSize::Dword)
}

#[test]
fn vpermt2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 643868047, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 125, 162, 143, 165, 96, 38], OperandSize::Dword)
}

#[test]
fn vpermt2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 213, 135, 125, 248], OperandSize::Qword)
}

#[test]
fn vpermt2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 181, 141, 125, 39], OperandSize::Qword)
}

#[test]
fn vpermt2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 170, 125, 204], OperandSize::Dword)
}

#[test]
fn vpermt2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 174, 125, 36, 242], OperandSize::Dword)
}

#[test]
fn vpermt2w_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 157, 175, 125, 202], OperandSize::Qword)
}

#[test]
fn vpermt2w_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 229, 175, 125, 44, 251], OperandSize::Qword)
}

#[test]
fn vpermt2w_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 201, 125, 196], OperandSize::Dword)
}

#[test]
fn vpermt2w_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 201, 125, 8], OperandSize::Dword)
}

#[test]
fn vpermt2w_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 189, 198, 125, 250], OperandSize::Qword)
}

#[test]
fn vpermt2w_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM9)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 181, 205, 125, 8], OperandSize::Qword)
}

