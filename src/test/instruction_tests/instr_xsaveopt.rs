use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsaveopt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEOPT, operand1: Some(IndirectScaledDisplaced(EDI, Two, 114488008, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 52, 125, 200, 242, 210, 6], OperandSize::Dword)
}

#[test]
fn xsaveopt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEOPT, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 911978132, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 180, 113, 148, 174, 91, 54], OperandSize::Qword)
}

