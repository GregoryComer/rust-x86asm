use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256rnds2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 195], OperandSize::Dword)
}

#[test]
fn sha256rnds2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 44, 215], OperandSize::Dword)
}

#[test]
fn sha256rnds2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 236], OperandSize::Qword)
}

#[test]
fn sha256rnds2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1481418717, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 156, 211, 221, 167, 76, 88], OperandSize::Qword)
}

