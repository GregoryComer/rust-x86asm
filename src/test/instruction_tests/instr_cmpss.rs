use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 210, 117], OperandSize::Dword)
}

#[test]
fn cmpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 40, 30], OperandSize::Dword)
}

#[test]
fn cmpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 223, 20], OperandSize::Qword)
}

#[test]
fn cmpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 498934646, Some(OperandSize::Dword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 44, 133, 118, 35, 189, 29, 50], OperandSize::Qword)
}

