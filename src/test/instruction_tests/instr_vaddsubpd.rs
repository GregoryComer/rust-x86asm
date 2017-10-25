use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 208, 204], OperandSize::Dword)
}

#[test]
fn vaddsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 464657450, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 208, 187, 42, 28, 178, 27], OperandSize::Dword)
}

#[test]
fn vaddsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 208, 210], OperandSize::Qword)
}

#[test]
fn vaddsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 1236262578, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 208, 180, 214, 178, 222, 175, 73], OperandSize::Qword)
}

#[test]
fn vaddsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 208, 203], OperandSize::Dword)
}

#[test]
fn vaddsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 841835691, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 208, 132, 79, 171, 100, 45, 50], OperandSize::Dword)
}

#[test]
fn vaddsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 208, 218], OperandSize::Qword)
}

#[test]
fn vaddsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 548106013, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 208, 156, 210, 29, 111, 171, 32], OperandSize::Qword)
}

