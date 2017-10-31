use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn haddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 230], OperandSize::Dword)
}

#[test]
fn haddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1311028805, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 44, 181, 69, 182, 36, 78], OperandSize::Dword)
}

#[test]
fn haddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 226], OperandSize::Qword)
}

#[test]
fn haddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RAX, 1572016588, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 124, 168, 204, 17, 179, 93], OperandSize::Qword)
}

