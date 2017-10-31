use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsxd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSXD, operand1: Some(Direct(RCX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 99, 204], OperandSize::Qword)
}

#[test]
fn movsxd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSXD, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1515690391, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 99, 156, 203, 151, 153, 87, 90], OperandSize::Qword)
}

