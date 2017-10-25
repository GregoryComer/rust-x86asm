use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesimc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 234], OperandSize::Dword)
}

#[test]
fn vaesimc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ESI, 1183662449, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 166, 113, 65, 141, 70], OperandSize::Dword)
}

#[test]
fn vaesimc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 204], OperandSize::Qword)
}

#[test]
fn vaesimc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 32], OperandSize::Qword)
}

