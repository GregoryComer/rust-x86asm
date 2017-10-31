use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshuflw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 251, 94], OperandSize::Dword)
}

#[test]
fn pshuflw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1182760018, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 188, 242, 82, 124, 127, 70, 46], OperandSize::Dword)
}

#[test]
fn pshuflw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 224, 38], OperandSize::Qword)
}

#[test]
fn pshuflw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 2141684049, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 132, 118, 81, 129, 167, 127, 88], OperandSize::Qword)
}

