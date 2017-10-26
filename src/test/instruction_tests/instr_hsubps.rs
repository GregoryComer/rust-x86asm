use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn hsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 250], OperandSize::Dword)
}

#[test]
fn hsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 54], OperandSize::Dword)
}

#[test]
fn hsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 198], OperandSize::Qword)
}

#[test]
fn hsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 60, 83], OperandSize::Qword)
}

