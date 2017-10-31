use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 249], OperandSize::Dword)
}

#[test]
fn addsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDI, 1119205864, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 191, 232, 185, 181, 66], OperandSize::Dword)
}

#[test]
fn addsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 226], OperandSize::Qword)
}

#[test]
fn addsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 799775942, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 164, 192, 198, 156, 171, 47], OperandSize::Qword)
}

