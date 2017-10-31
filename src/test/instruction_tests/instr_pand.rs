use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 218], OperandSize::Dword)
}

#[test]
fn pand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 4, 203], OperandSize::Dword)
}

#[test]
fn pand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 249], OperandSize::Qword)
}

#[test]
fn pand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 408916683, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 20, 149, 203, 146, 95, 24], OperandSize::Qword)
}

#[test]
fn pand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 235], OperandSize::Dword)
}

#[test]
fn pand_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1334936028, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 148, 254, 220, 129, 145, 79], OperandSize::Dword)
}

#[test]
fn pand_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 194], OperandSize::Qword)
}

#[test]
fn pand_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 20, 242], OperandSize::Qword)
}

