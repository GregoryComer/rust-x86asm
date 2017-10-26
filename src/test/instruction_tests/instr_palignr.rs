use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn palignr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 214, 73], OperandSize::Dword)
}

#[test]
fn palignr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1514194422, Some(OperandSize::Qword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 28, 181, 246, 197, 64, 90, 33], OperandSize::Dword)
}

#[test]
fn palignr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 220, 30], OperandSize::Qword)
}

#[test]
fn palignr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 301546183, Some(OperandSize::Qword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 156, 216, 199, 58, 249, 17, 16], OperandSize::Qword)
}

#[test]
fn palignr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 196, 1], OperandSize::Dword)
}

#[test]
fn palignr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 722564171, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 158, 75, 116, 17, 43, 92], OperandSize::Dword)
}

#[test]
fn palignr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 195, 94], OperandSize::Qword)
}

#[test]
fn palignr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1372635638, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 12, 125, 246, 193, 208, 81, 40], OperandSize::Qword)
}

