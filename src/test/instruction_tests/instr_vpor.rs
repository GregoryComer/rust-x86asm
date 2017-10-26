use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 235, 207], OperandSize::Dword)
}

#[test]
fn vpor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDI, 764503083, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 235, 143, 43, 100, 145, 45], OperandSize::Dword)
}

#[test]
fn vpor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 235, 233], OperandSize::Qword)
}

#[test]
fn vpor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 235, 12, 215], OperandSize::Qword)
}

#[test]
fn vpor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 235, 241], OperandSize::Dword)
}

#[test]
fn vpor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1695024801, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 235, 148, 251, 161, 6, 8, 101], OperandSize::Dword)
}

#[test]
fn vpor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 235, 245], OperandSize::Qword)
}

#[test]
fn vpor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RDI, 1800199154, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 235, 175, 242, 219, 76, 107], OperandSize::Qword)
}

