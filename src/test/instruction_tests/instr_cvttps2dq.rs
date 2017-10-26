use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 221], OperandSize::Dword)
}

#[test]
fn cvttps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 12, 182], OperandSize::Dword)
}

#[test]
fn cvttps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 222], OperandSize::Qword)
}

#[test]
fn cvttps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 91, 15], OperandSize::Qword)
}

