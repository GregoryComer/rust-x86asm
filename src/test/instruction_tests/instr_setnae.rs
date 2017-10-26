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
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 11694, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 131, 174, 45], OperandSize::Word)
}

#[test]
fn setnae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Dword)
}

#[test]
fn setnae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 143], OperandSize::Dword)
}

#[test]
fn setnae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Qword)
}

#[test]
fn setnae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectDisplaced(RBX, 1858232177, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 131, 113, 95, 194, 110], OperandSize::Qword)
}

#[test]
fn setnae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Qword)
}

#[test]
fn setnae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNAE, operand1: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 128], OperandSize::Qword)
}

