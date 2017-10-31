use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcompresspd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 138, 227], OperandSize::Dword)
}

#[test]
fn vcompresspd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectDisplaced(EBX, 839390845, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 8, 138, 155, 125, 22, 8, 50], OperandSize::Dword)
}

#[test]
fn vcompresspd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 253, 137, 138, 248], OperandSize::Qword)
}

#[test]
fn vcompresspd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 740430659, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 8, 138, 156, 64, 67, 19, 34, 44], OperandSize::Qword)
}

#[test]
fn vcompresspd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 138, 210], OperandSize::Dword)
}

#[test]
fn vcompresspd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 40, 138, 8], OperandSize::Dword)
}

#[test]
fn vcompresspd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 253, 171, 138, 222], OperandSize::Qword)
}

#[test]
fn vcompresspd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 253, 40, 138, 28, 130], OperandSize::Qword)
}

#[test]
fn vcompresspd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 138, 246], OperandSize::Dword)
}

#[test]
fn vcompresspd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 896588793, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 138, 180, 143, 249, 219, 112, 53], OperandSize::Dword)
}

#[test]
fn vcompresspd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 253, 205, 138, 221], OperandSize::Qword)
}

#[test]
fn vcompresspd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectDisplaced(RBX, 1866353354, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 138, 187, 202, 74, 62, 111], OperandSize::Qword)
}

