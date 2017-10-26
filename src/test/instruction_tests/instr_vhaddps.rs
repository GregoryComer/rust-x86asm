use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhaddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 124, 197], OperandSize::Dword)
}

#[test]
fn vhaddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 298636068, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 124, 180, 254, 36, 211, 204, 17], OperandSize::Dword)
}

#[test]
fn vhaddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 124, 209], OperandSize::Qword)
}

#[test]
fn vhaddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 124, 6], OperandSize::Qword)
}

#[test]
fn vhaddps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 215, 124, 226], OperandSize::Dword)
}

#[test]
fn vhaddps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1158398564, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 247, 124, 140, 114, 100, 194, 11, 69], OperandSize::Dword)
}

#[test]
fn vhaddps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 199, 124, 233], OperandSize::Qword)
}

#[test]
fn vhaddps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 247, 124, 38], OperandSize::Qword)
}

