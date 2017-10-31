use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rdseed_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RDSEED, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 199, 252], OperandSize::Dword)
}

#[test]
fn rdseed_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RDSEED, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 199, 250], OperandSize::Qword)
}

#[test]
fn rdseed_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RDSEED, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 252], OperandSize::Dword)
}

#[test]
fn rdseed_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RDSEED, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 249], OperandSize::Qword)
}

#[test]
fn rdseed_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RDSEED, operand1: Some(Direct(RBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 199, 253], OperandSize::Qword)
}

