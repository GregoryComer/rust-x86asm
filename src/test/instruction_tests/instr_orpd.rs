use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn orpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 204], OperandSize::Dword)
}

#[test]
fn orpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 1851431764, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 180, 201, 84, 155, 90, 110], OperandSize::Dword)
}

#[test]
fn orpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 211], OperandSize::Qword)
}

#[test]
fn orpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 35], OperandSize::Qword)
}

