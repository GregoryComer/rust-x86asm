use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xrstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(Indirect(SI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 44], OperandSize::Word)
}

#[test]
fn xrstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 1209067044, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 172, 222, 36, 230, 16, 72], OperandSize::Dword)
}

#[test]
fn xrstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(Indirect(RCX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 41], OperandSize::Qword)
}

