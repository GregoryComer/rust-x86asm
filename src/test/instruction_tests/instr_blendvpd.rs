use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendvpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 210], OperandSize::Dword)
}

#[test]
fn blendvpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 68724845, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 28, 253, 109, 168, 24, 4], OperandSize::Dword)
}

#[test]
fn blendvpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 236], OperandSize::Qword)
}

#[test]
fn blendvpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 60, 210], OperandSize::Qword)
}

