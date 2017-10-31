use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1msg2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 219], OperandSize::Dword)
}

#[test]
fn sha1msg2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 28, 153], OperandSize::Dword)
}

#[test]
fn sha1msg2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 250], OperandSize::Qword)
}

#[test]
fn sha1msg2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 3], OperandSize::Qword)
}

