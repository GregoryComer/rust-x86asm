use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 241], OperandSize::Dword)
}

#[test]
fn punpckhwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(EAX, 570260300, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 152, 76, 123, 253, 33], OperandSize::Dword)
}

#[test]
fn punpckhwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 241], OperandSize::Qword)
}

#[test]
fn punpckhwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 971980986, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 52, 69, 186, 64, 239, 57], OperandSize::Qword)
}

#[test]
fn punpckhwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 236], OperandSize::Dword)
}

#[test]
fn punpckhwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 12, 211], OperandSize::Dword)
}

#[test]
fn punpckhwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 212], OperandSize::Qword)
}

#[test]
fn punpckhwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDI, 1363216020, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 167, 148, 6, 65, 81], OperandSize::Qword)
}

