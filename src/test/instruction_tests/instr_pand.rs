use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 251], OperandSize::Dword)
}

#[test]
fn pand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1865376048, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 44, 93, 48, 97, 47, 111], OperandSize::Dword)
}

#[test]
fn pand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 216], OperandSize::Qword)
}

#[test]
fn pand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(RBX, 793134373, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 139, 37, 69, 70, 47], OperandSize::Qword)
}

#[test]
fn pand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 247], OperandSize::Dword)
}

#[test]
fn pand_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 48], OperandSize::Dword)
}

#[test]
fn pand_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 254], OperandSize::Qword)
}

#[test]
fn pand_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDX, 598290735, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 170, 47, 49, 169, 35], OperandSize::Qword)
}

