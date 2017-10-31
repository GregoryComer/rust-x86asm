use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 240], OperandSize::Dword)
}

#[test]
fn minps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 729116784, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 164, 73, 112, 112, 117, 43], OperandSize::Dword)
}

#[test]
fn minps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 240], OperandSize::Qword)
}

#[test]
fn minps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 60, 158], OperandSize::Qword)
}

