use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256rnds2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 203], OperandSize::Dword)
}

#[test]
fn sha256rnds2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 60, 155], OperandSize::Dword)
}

#[test]
fn sha256rnds2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 231], OperandSize::Qword)
}

#[test]
fn sha256rnds2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 14], OperandSize::Qword)
}

