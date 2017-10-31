use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 195], OperandSize::Dword)
}

#[test]
fn pmaxub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1681410403, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 12, 189, 99, 73, 56, 100], OperandSize::Dword)
}

#[test]
fn pmaxub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 247], OperandSize::Qword)
}

#[test]
fn pmaxub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(MM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 222, 23], OperandSize::Qword)
}

#[test]
fn pmaxub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 229], OperandSize::Dword)
}

#[test]
fn pmaxub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 77476677, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 164, 214, 69, 51, 158, 4], OperandSize::Dword)
}

#[test]
fn pmaxub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 247], OperandSize::Qword)
}

#[test]
fn pmaxub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUB, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 222, 3], OperandSize::Qword)
}

