use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 143, 125, 225], OperandSize::Dword)
}

#[test]
fn vpermt2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 125, 10], OperandSize::Dword)
}

#[test]
fn vpermt2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 181, 142, 125, 204], OperandSize::Qword)
}

#[test]
fn vpermt2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 125, 12, 88], OperandSize::Qword)
}

#[test]
fn vpermt2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 125, 253], OperandSize::Dword)
}

#[test]
fn vpermt2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 173, 125, 4, 86], OperandSize::Dword)
}

#[test]
fn vpermt2w_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 165, 166, 125, 227], OperandSize::Qword)
}

#[test]
fn vpermt2w_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RDI, 303451928, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 237, 162, 125, 183, 24, 79, 22, 18], OperandSize::Qword)
}

#[test]
fn vpermt2w_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 202, 125, 216], OperandSize::Dword)
}

#[test]
fn vpermt2w_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 198856768, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 207, 125, 28, 125, 64, 80, 218, 11], OperandSize::Dword)
}

#[test]
fn vpermt2w_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 221, 205, 125, 205], OperandSize::Qword)
}

#[test]
fn vpermt2w_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 128504076, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 207, 125, 60, 125, 12, 209, 168, 7], OperandSize::Qword)
}

