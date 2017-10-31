use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 235, 202], OperandSize::Dword)
}

#[test]
fn vpor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 233772666, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 235, 44, 221, 122, 22, 239, 13], OperandSize::Dword)
}

#[test]
fn vpor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 235, 233], OperandSize::Qword)
}

#[test]
fn vpor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 235, 12, 154], OperandSize::Qword)
}

#[test]
fn vpor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 235, 251], OperandSize::Dword)
}

#[test]
fn vpor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 235, 4, 136], OperandSize::Dword)
}

#[test]
fn vpor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 235, 215], OperandSize::Qword)
}

#[test]
fn vpor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 235, 7], OperandSize::Qword)
}

