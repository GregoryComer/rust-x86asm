use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesimc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 231], OperandSize::Dword)
}

#[test]
fn vaesimc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1071062867, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 164, 83, 83, 31, 215, 63], OperandSize::Dword)
}

#[test]
fn vaesimc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 232], OperandSize::Qword)
}

#[test]
fn vaesimc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 339572892, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 20, 141, 156, 120, 61, 20], OperandSize::Qword)
}

