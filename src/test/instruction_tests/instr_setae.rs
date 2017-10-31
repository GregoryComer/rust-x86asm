use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Word)
}

#[test]
fn setae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 21638, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 130, 134, 84], OperandSize::Word)
}

#[test]
fn setae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Dword)
}

#[test]
fn setae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectDisplaced(ECX, 600486142, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 129, 254, 176, 202, 35], OperandSize::Dword)
}

#[test]
fn setae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Qword)
}

#[test]
fn setae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectDisplaced(RDI, 905069438, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 135, 126, 67, 242, 53], OperandSize::Qword)
}

#[test]
fn setae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Qword)
}

#[test]
fn setae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectDisplaced(RDX, 1866104408, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 130, 88, 126, 58, 111], OperandSize::Qword)
}

