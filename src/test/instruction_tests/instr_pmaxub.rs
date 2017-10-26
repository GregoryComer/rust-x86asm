use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 214], OperandSize::Dword)
}

#[test]
fn pmaxub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 293946388, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 156, 223, 20, 68, 133, 17], OperandSize::Dword)
}

#[test]
fn pmaxub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 203], OperandSize::Qword)
}

#[test]
fn pmaxub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(RDX, 1836628207, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 170, 239, 184, 120, 109], OperandSize::Qword)
}

#[test]
fn pmaxub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 221], OperandSize::Dword)
}

#[test]
fn pmaxub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1950706862, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 4, 125, 174, 108, 69, 116], OperandSize::Dword)
}

#[test]
fn pmaxub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 201], OperandSize::Qword)
}

#[test]
fn pmaxub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 60, 178], OperandSize::Qword)
}

