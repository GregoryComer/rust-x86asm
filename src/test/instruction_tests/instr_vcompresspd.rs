use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcompresspd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 138, 217], OperandSize::Dword)
}

#[test]
fn vcompresspd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledDisplaced(ECX, Two, 934962032, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 8, 138, 60, 77, 112, 99, 186, 55], OperandSize::Dword)
}

#[test]
fn vcompresspd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 253, 140, 138, 248], OperandSize::Qword)
}

#[test]
fn vcompresspd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 253, 8, 138, 12, 246], OperandSize::Qword)
}

#[test]
fn vcompresspd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 138, 221], OperandSize::Dword)
}

#[test]
fn vcompresspd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectDisplaced(ESI, 2069327149, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 40, 138, 166, 45, 109, 87, 123], OperandSize::Dword)
}

#[test]
fn vcompresspd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 253, 169, 138, 196], OperandSize::Qword)
}

#[test]
fn vcompresspd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 601062562, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 253, 40, 138, 164, 184, 162, 124, 211, 35], OperandSize::Qword)
}

#[test]
fn vcompresspd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 138, 239], OperandSize::Dword)
}

#[test]
fn vcompresspd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 138, 12, 187], OperandSize::Dword)
}

#[test]
fn vcompresspd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 253, 203, 138, 241], OperandSize::Qword)
}

#[test]
fn vcompresspd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1001750862, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 72, 138, 12, 77, 78, 129, 181, 59], OperandSize::Qword)
}

