use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pandn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 203], OperandSize::Dword)
}

#[test]
fn pandn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1444593347, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 188, 158, 195, 190, 26, 86], OperandSize::Dword)
}

#[test]
fn pandn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 231], OperandSize::Qword)
}

#[test]
fn pandn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1108812989, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 60, 85, 189, 36, 23, 66], OperandSize::Qword)
}

#[test]
fn pandn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 206], OperandSize::Dword)
}

#[test]
fn pandn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 4, 71], OperandSize::Dword)
}

#[test]
fn pandn_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 247], OperandSize::Qword)
}

#[test]
fn pandn_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 740151163, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 36, 221, 123, 207, 29, 44], OperandSize::Qword)
}

