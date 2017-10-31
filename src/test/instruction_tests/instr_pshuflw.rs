use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshuflw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 250, 0], OperandSize::Dword)
}

#[test]
fn pshuflw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 62737234, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 156, 119, 82, 75, 189, 3, 29], OperandSize::Dword)
}

#[test]
fn pshuflw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 218, 20], OperandSize::Qword)
}

#[test]
fn pshuflw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RCX, 126780103, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 137, 199, 130, 142, 7, 13], OperandSize::Qword)
}

