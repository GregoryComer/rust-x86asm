use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 230], OperandSize::Dword)
}

#[test]
fn pcmpgtb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 60, 217], OperandSize::Dword)
}

#[test]
fn pcmpgtb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 213], OperandSize::Qword)
}

#[test]
fn pcmpgtb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RDI, 176289802, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 135, 10, 248, 129, 10], OperandSize::Qword)
}

#[test]
fn pcmpgtb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 252], OperandSize::Dword)
}

#[test]
fn pcmpgtb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDX, 639280342, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 186, 214, 164, 26, 38], OperandSize::Dword)
}

#[test]
fn pcmpgtb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 235], OperandSize::Qword)
}

#[test]
fn pcmpgtb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 28, 199], OperandSize::Qword)
}

