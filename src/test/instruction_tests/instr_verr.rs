use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn verr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 229], OperandSize::Word)
}

#[test]
fn verr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 36], OperandSize::Word)
}

#[test]
fn verr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 227], OperandSize::Dword)
}

#[test]
fn verr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1168269032, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 36, 117, 232, 94, 162, 69], OperandSize::Dword)
}

#[test]
fn verr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 231], OperandSize::Qword)
}

#[test]
fn verr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VERR, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1833539953, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 164, 247, 113, 153, 73, 109], OperandSize::Qword)
}

