use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 245], OperandSize::Dword)
}

#[test]
fn pminuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ECX, 1975184600, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 177, 216, 236, 186, 117], OperandSize::Dword)
}

#[test]
fn pminuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 240], OperandSize::Qword)
}

#[test]
fn pminuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1284333608, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 140, 182, 40, 96, 141, 76], OperandSize::Qword)
}

