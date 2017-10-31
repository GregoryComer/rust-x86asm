use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pandn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 242], OperandSize::Dword)
}

#[test]
fn pandn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 901000392, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 52, 141, 200, 44, 180, 53], OperandSize::Dword)
}

#[test]
fn pandn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 253], OperandSize::Qword)
}

#[test]
fn pandn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 107841894, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 60, 77, 102, 137, 109, 6], OperandSize::Qword)
}

#[test]
fn pandn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 229], OperandSize::Dword)
}

#[test]
fn pandn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 28, 146], OperandSize::Dword)
}

#[test]
fn pandn_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 251], OperandSize::Qword)
}

#[test]
fn pandn_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1417184977, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 12, 213, 209, 134, 120, 84], OperandSize::Qword)
}

