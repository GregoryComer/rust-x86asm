use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 192], OperandSize::Dword)
}

#[test]
fn pmaxub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(EDX, 1962140120, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 146, 216, 225, 243, 116], OperandSize::Dword)
}

#[test]
fn pmaxub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 214], OperandSize::Qword)
}

#[test]
fn pmaxub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 60, 159], OperandSize::Qword)
}

#[test]
fn pmaxub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 196], OperandSize::Dword)
}

#[test]
fn pmaxub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1630884210, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 44, 213, 114, 81, 53, 97], OperandSize::Dword)
}

#[test]
fn pmaxub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 216], OperandSize::Qword)
}

#[test]
fn pmaxub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1538470445, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 156, 203, 45, 50, 179, 91], OperandSize::Qword)
}

