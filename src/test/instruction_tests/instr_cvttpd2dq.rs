use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 224], OperandSize::Dword)
}

#[test]
fn cvttpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ESI, 763793211, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 190, 59, 143, 134, 45], OperandSize::Dword)
}

#[test]
fn cvttpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 240], OperandSize::Qword)
}

#[test]
fn cvttpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RBX, 2026325369, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 179, 121, 69, 199, 120], OperandSize::Qword)
}

