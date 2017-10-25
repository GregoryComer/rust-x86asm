use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 193], OperandSize::Dword)
}

#[test]
fn xorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1505641279, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 4, 85, 63, 67, 190, 89], OperandSize::Dword)
}

#[test]
fn xorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 245], OperandSize::Qword)
}

#[test]
fn xorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RCX, 1211170620, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 177, 60, 255, 48, 72], OperandSize::Qword)
}

