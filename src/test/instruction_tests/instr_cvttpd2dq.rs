use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 254], OperandSize::Dword)
}

#[test]
fn cvttpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 52, 200], OperandSize::Dword)
}

#[test]
fn cvttpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 210], OperandSize::Qword)
}

#[test]
fn cvttpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 230, 47], OperandSize::Qword)
}

