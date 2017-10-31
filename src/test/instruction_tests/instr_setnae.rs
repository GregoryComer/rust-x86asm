use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Word)
}

#[test]
fn setnae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 2], OperandSize::Word)
}

#[test]
fn setnae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Dword)
}

#[test]
fn setnae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 0], OperandSize::Dword)
}

#[test]
fn setnae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

#[test]
fn setnae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 6], OperandSize::Qword)
}

#[test]
fn setnae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Qword)
}

#[test]
fn setnae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledDisplaced(RCX, Four, 2008889841, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 141, 241, 57, 189, 119], OperandSize::Qword)
}

