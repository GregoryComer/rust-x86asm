use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 233], OperandSize::Dword)
}

#[test]
fn pand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EBX, 498449789, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 187, 125, 189, 181, 29], OperandSize::Dword)
}

#[test]
fn pand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 229], OperandSize::Qword)
}

#[test]
fn pand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1936753183, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 12, 133, 31, 130, 112, 115], OperandSize::Qword)
}

#[test]
fn pand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 249], OperandSize::Dword)
}

#[test]
fn pand_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 12, 113], OperandSize::Dword)
}

#[test]
fn pand_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 212], OperandSize::Qword)
}

#[test]
fn pand_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 44, 135], OperandSize::Qword)
}

