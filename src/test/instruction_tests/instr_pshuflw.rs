use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshuflw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 236, 81], OperandSize::Dword)
}

#[test]
fn pshuflw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 2038842590, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 172, 127, 222, 68, 134, 121, 31], OperandSize::Dword)
}

#[test]
fn pshuflw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 207, 74], OperandSize::Qword)
}

#[test]
fn pshuflw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 51, 123], OperandSize::Qword)
}

