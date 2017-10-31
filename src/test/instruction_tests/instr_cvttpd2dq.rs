use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 199], OperandSize::Dword)
}

#[test]
fn cvttpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDX, 1316219172, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 138, 36, 233, 115, 78], OperandSize::Dword)
}

#[test]
fn cvttpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 199], OperandSize::Qword)
}

#[test]
fn cvttpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1788699669, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 148, 66, 21, 100, 157, 106], OperandSize::Qword)
}

