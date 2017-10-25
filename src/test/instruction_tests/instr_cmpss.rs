use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 196, 13], OperandSize::Dword)
}

#[test]
fn cmpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EAX, 1223477074, Some(OperandSize::Dword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 168, 82, 199, 236, 72, 27], OperandSize::Dword)
}

#[test]
fn cmpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 228, 114], OperandSize::Qword)
}

#[test]
fn cmpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPSS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 194, 10, 94], OperandSize::Qword)
}

