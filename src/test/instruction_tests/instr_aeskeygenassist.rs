use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aeskeygenassist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 192, 94], OperandSize::Dword)
}

#[test]
fn aeskeygenassist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1285244837, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 36, 181, 165, 71, 155, 76, 124], OperandSize::Dword)
}

#[test]
fn aeskeygenassist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 230, 91], OperandSize::Qword)
}

#[test]
fn aeskeygenassist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 16, 4], OperandSize::Qword)
}

