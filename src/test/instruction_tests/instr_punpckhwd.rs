use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 197], OperandSize::Dword)
}

#[test]
fn punpckhwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 54], OperandSize::Dword)
}

#[test]
fn punpckhwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 230], OperandSize::Qword)
}

#[test]
fn punpckhwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 25], OperandSize::Qword)
}

#[test]
fn punpckhwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 254], OperandSize::Dword)
}

#[test]
fn punpckhwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1838427812, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 140, 114, 164, 46, 148, 109], OperandSize::Dword)
}

#[test]
fn punpckhwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 202], OperandSize::Qword)
}

#[test]
fn punpckhwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 213790612, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 20, 117, 148, 47, 190, 12], OperandSize::Qword)
}

