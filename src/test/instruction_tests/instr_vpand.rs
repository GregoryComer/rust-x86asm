use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 219, 210], OperandSize::Dword)
}

#[test]
fn vpand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1017550118, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 219, 172, 158, 38, 149, 166, 60], OperandSize::Dword)
}

#[test]
fn vpand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 219, 247], OperandSize::Qword)
}

#[test]
fn vpand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 219, 28, 240], OperandSize::Qword)
}

#[test]
fn vpand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 219, 227], OperandSize::Dword)
}

#[test]
fn vpand_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1548023224, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 219, 132, 191, 184, 245, 68, 92], OperandSize::Dword)
}

#[test]
fn vpand_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 219, 228], OperandSize::Qword)
}

#[test]
fn vpand_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAND, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RCX, 1915069617, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 219, 169, 177, 164, 37, 114], OperandSize::Qword)
}

