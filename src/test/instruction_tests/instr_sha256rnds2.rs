use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256rnds2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 240], OperandSize::Dword)
}

#[test]
fn sha256rnds2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ECX, 912664446, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 153, 126, 39, 102, 54], OperandSize::Dword)
}

#[test]
fn sha256rnds2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 229], OperandSize::Qword)
}

#[test]
fn sha256rnds2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 2056233338, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 44, 197, 122, 161, 143, 122], OperandSize::Qword)
}

