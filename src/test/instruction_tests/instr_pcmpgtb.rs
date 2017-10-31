use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 247], OperandSize::Dword)
}

#[test]
fn pcmpgtb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1295807935, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 4, 253, 191, 117, 60, 77], OperandSize::Dword)
}

#[test]
fn pcmpgtb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 203], OperandSize::Qword)
}

#[test]
fn pcmpgtb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 12, 176], OperandSize::Qword)
}

#[test]
fn pcmpgtb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 224], OperandSize::Dword)
}

#[test]
fn pcmpgtb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1468485233, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 188, 87, 113, 78, 135, 87], OperandSize::Dword)
}

#[test]
fn pcmpgtb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 218], OperandSize::Qword)
}

#[test]
fn pcmpgtb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 12, 247], OperandSize::Qword)
}

