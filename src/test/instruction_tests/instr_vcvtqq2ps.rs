use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtqq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 138, 91, 216], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1331394699, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 140, 91, 132, 144, 139, 120, 91, 79], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 252, 140, 91, 240], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 309010069, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 142, 91, 52, 189, 149, 30, 107, 18], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 174, 91, 198], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 175, 91, 15], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 252, 171, 91, 222], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 252, 171, 91, 36, 115], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 252, 153, 91, 204], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 207, 91, 36, 216], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 252, 157, 91, 244], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 943057021, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 252, 203, 91, 60, 157, 125, 232, 53, 56], OperandSize::Qword)
}

