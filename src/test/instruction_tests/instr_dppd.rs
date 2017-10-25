use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 200, 80], OperandSize::Dword)
}

#[test]
fn dppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 747275502, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 172, 249, 238, 132, 138, 44, 81], OperandSize::Dword)
}

#[test]
fn dppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 226, 112], OperandSize::Qword)
}

#[test]
fn dppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1157982571, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 148, 123, 107, 105, 5, 69, 111], OperandSize::Qword)
}

