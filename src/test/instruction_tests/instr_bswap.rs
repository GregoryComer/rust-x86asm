use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bswap_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSWAP, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 203], OperandSize::Word)
}

#[test]
fn bswap_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSWAP, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 201], OperandSize::Dword)
}

#[test]
fn bswap_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSWAP, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 205], OperandSize::Qword)
}

#[test]
fn bswap_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSWAP, operand1: Some(Direct(RBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 205], OperandSize::Qword)
}

