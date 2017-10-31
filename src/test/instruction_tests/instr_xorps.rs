use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 235], OperandSize::Dword)
}

#[test]
fn xorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 512013486, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 140, 86, 174, 180, 132, 30], OperandSize::Dword)
}

#[test]
fn xorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 211], OperandSize::Qword)
}

#[test]
fn xorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1952256658, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 172, 215, 146, 18, 93, 116], OperandSize::Qword)
}

