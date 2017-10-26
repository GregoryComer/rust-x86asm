use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn orps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 229], OperandSize::Dword)
}

#[test]
fn orps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1782895493, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 44, 181, 133, 211, 68, 106], OperandSize::Dword)
}

#[test]
fn orps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 237], OperandSize::Qword)
}

#[test]
fn orps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RCX, 14332804, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 86, 153, 132, 179, 218, 0], OperandSize::Qword)
}

