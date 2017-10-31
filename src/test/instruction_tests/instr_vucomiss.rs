use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vucomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 251], OperandSize::Dword)
}

#[test]
fn vucomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 12, 82], OperandSize::Dword)
}

#[test]
fn vucomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 246], OperandSize::Qword)
}

#[test]
fn vucomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RSI, 374000504, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 134, 120, 203, 74, 22], OperandSize::Qword)
}

#[test]
fn vucomiss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 46, 248], OperandSize::Dword)
}

#[test]
fn vucomiss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 55765318, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 20, 221, 70, 233, 82, 3], OperandSize::Dword)
}

#[test]
fn vucomiss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 225, 124, 24, 46, 240], OperandSize::Qword)
}

#[test]
fn vucomiss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM16)), operand2: Some(IndirectDisplaced(RBX, 1278735113, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 8, 46, 131, 9, 243, 55, 76], OperandSize::Qword)
}

