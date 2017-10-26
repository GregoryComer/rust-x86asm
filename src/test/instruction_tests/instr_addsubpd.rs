use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 253], OperandSize::Dword)
}

#[test]
fn addsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 1656457282, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 158, 66, 136, 187, 98], OperandSize::Dword)
}

#[test]
fn addsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 222], OperandSize::Qword)
}

#[test]
fn addsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RSI, 938007922, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 142, 114, 221, 232, 55], OperandSize::Qword)
}

