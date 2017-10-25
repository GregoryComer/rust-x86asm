use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 235, 240], OperandSize::Dword)
}

#[test]
fn vpor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 235, 28, 246], OperandSize::Dword)
}

#[test]
fn vpor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 235, 200], OperandSize::Qword)
}

#[test]
fn vpor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 235, 30], OperandSize::Qword)
}

#[test]
fn vpor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 235, 195], OperandSize::Dword)
}

#[test]
fn vpor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 235, 36, 114], OperandSize::Dword)
}

#[test]
fn vpor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 235, 205], OperandSize::Qword)
}

#[test]
fn vpor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 235, 20, 67], OperandSize::Qword)
}

