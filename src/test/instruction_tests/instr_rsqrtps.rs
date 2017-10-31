use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 214], OperandSize::Dword)
}

#[test]
fn rsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1571527684, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 60, 69, 4, 156, 171, 93], OperandSize::Dword)
}

#[test]
fn rsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 196], OperandSize::Qword)
}

#[test]
fn rsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 82, 28, 146], OperandSize::Qword)
}

