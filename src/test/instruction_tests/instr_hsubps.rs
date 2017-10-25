use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn hsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 193], OperandSize::Dword)
}

#[test]
fn hsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 560503143, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 36, 245, 103, 153, 104, 33], OperandSize::Dword)
}

#[test]
fn hsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 194], OperandSize::Qword)
}

#[test]
fn hsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 125, 26], OperandSize::Qword)
}

