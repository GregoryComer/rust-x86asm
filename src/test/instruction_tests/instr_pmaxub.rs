use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 215], OperandSize::Dword)
}

#[test]
fn pmaxub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1388092783, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 60, 157, 111, 157, 188, 82], OperandSize::Dword)
}

#[test]
fn pmaxub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 195], OperandSize::Qword)
}

#[test]
fn pmaxub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 31], OperandSize::Qword)
}

#[test]
fn pmaxub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 234], OperandSize::Dword)
}

#[test]
fn pmaxub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EBX, 250636776, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 179, 232, 105, 240, 14], OperandSize::Dword)
}

#[test]
fn pmaxub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 193], OperandSize::Qword)
}

#[test]
fn pmaxub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RAX, 91248467, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 136, 83, 87, 112, 5], OperandSize::Qword)
}

