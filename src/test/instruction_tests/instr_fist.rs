use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 18], OperandSize::Word)
}

#[test]
fn fist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1171204321, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 148, 184, 225, 40, 207, 69], OperandSize::Dword)
}

#[test]
fn fist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 20, 210], OperandSize::Qword)
}

#[test]
fn fist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 20], OperandSize::Word)
}

#[test]
fn fist_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 332201873, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 148, 191, 145, 255, 204, 19], OperandSize::Dword)
}

#[test]
fn fist_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1013316779, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 148, 87, 171, 252, 101, 60], OperandSize::Qword)
}

