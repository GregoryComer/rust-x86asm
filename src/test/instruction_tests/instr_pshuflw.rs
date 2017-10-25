use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshuflw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 204, 68], OperandSize::Dword)
}

#[test]
fn pshuflw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1411564962, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 188, 64, 162, 197, 34, 84, 70], OperandSize::Dword)
}

#[test]
fn pshuflw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 248, 103], OperandSize::Qword)
}

#[test]
fn pshuflw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 62, 84], OperandSize::Qword)
}

