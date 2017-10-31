use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsaveopt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEOPT, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1407384179, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 180, 187, 115, 250, 226, 83], OperandSize::Dword)
}

#[test]
fn xsaveopt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEOPT, operand1: Some(Indirect(RAX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 48], OperandSize::Qword)
}

