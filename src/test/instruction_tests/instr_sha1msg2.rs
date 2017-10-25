use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha1msg2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 240], OperandSize::Dword)
}

#[test]
fn sha1msg2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 2130318579, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 12, 189, 243, 20, 250, 126], OperandSize::Dword)
}

#[test]
fn sha1msg2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 221], OperandSize::Qword)
}

#[test]
fn sha1msg2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG2, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 202, 23], OperandSize::Qword)
}

