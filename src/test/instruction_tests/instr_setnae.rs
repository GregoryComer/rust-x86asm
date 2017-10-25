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
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectDisplaced(BP, 20169, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 134, 201, 78], OperandSize::Word)
}

#[test]
fn setnae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Dword)
}

#[test]
fn setnae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 3], OperandSize::Dword)
}

#[test]
fn setnae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Qword)
}

#[test]
fn setnae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 190], OperandSize::Qword)
}

#[test]
fn setnae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Qword)
}

#[test]
fn setnae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 2011392775, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 132, 199, 7, 107, 227, 119], OperandSize::Qword)
}

