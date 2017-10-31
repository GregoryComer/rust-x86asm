use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Word)
}

#[test]
fn setnae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 235, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 129, 235, 0], OperandSize::Word)
}

#[test]
fn setnae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Dword)
}

#[test]
fn setnae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectDisplaced(EAX, 781688505, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 128, 185, 158, 151, 46], OperandSize::Dword)
}

#[test]
fn setnae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

#[test]
fn setnae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectDisplaced(RCX, 583218464, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 129, 32, 53, 195, 34], OperandSize::Qword)
}

#[test]
fn setnae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

#[test]
fn setnae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1054341356, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 205, 236, 248, 215, 62], OperandSize::Qword)
}

