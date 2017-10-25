use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 236], OperandSize::Dword)
}

#[test]
fn cvtps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 30], OperandSize::Dword)
}

#[test]
fn cvtps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 248], OperandSize::Qword)
}

#[test]
fn cvtps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RDI, 1782784836, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 91, 191, 68, 35, 67, 106], OperandSize::Qword)
}

