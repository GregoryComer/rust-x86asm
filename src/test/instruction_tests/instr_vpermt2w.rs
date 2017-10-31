use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 142, 125, 229], OperandSize::Dword)
}

#[test]
fn vpermt2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 125, 4, 130], OperandSize::Dword)
}

#[test]
fn vpermt2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 157, 138, 125, 248], OperandSize::Qword)
}

#[test]
fn vpermt2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 157, 143, 125, 4, 203], OperandSize::Qword)
}

#[test]
fn vpermt2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 174, 125, 242], OperandSize::Dword)
}

#[test]
fn vpermt2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDI, 1832567498, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 173, 125, 183, 202, 194, 58, 109], OperandSize::Dword)
}

#[test]
fn vpermt2w_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 197, 173, 125, 208], OperandSize::Qword)
}

#[test]
fn vpermt2w_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 892662441, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 245, 175, 125, 12, 133, 169, 242, 52, 53], OperandSize::Qword)
}

#[test]
fn vpermt2w_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 203, 125, 246], OperandSize::Dword)
}

#[test]
fn vpermt2w_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 205, 125, 60, 72], OperandSize::Dword)
}

#[test]
fn vpermt2w_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 165, 207, 125, 193], OperandSize::Qword)
}

#[test]
fn vpermt2w_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2W, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM9)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 181, 203, 125, 3], OperandSize::Qword)
}

