use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Word)
}

#[test]
fn setc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectDisplaced(SI, 214, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 132, 214, 0], OperandSize::Word)
}

#[test]
fn setc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Dword)
}

#[test]
fn setc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 713028692, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 197, 84, 244, 127, 42], OperandSize::Dword)
}

#[test]
fn setc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Qword)
}

#[test]
fn setc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 122], OperandSize::Qword)
}

#[test]
fn setc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Qword)
}

#[test]
fn setc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectDisplaced(RCX, 1062100690, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 129, 210, 94, 78, 63], OperandSize::Qword)
}

