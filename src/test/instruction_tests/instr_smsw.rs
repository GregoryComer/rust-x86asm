use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn smsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 227], OperandSize::Word)
}

#[test]
fn smsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 34], OperandSize::Word)
}

#[test]
fn smsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 1, 231], OperandSize::Dword)
}

#[test]
fn smsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 36, 201], OperandSize::Dword)
}

#[test]
fn smsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 1, 230], OperandSize::Qword)
}

#[test]
fn smsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectDisplaced(RBX, 121185948, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 163, 156, 38, 57, 7], OperandSize::Qword)
}

#[test]
fn smsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 1, 225], OperandSize::Word)
}

#[test]
fn smsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectDisplaced(DI, 17700, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 165, 36, 69], OperandSize::Word)
}

#[test]
fn smsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 228], OperandSize::Dword)
}

#[test]
fn smsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 36, 118], OperandSize::Dword)
}

#[test]
fn smsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 230], OperandSize::Qword)
}

#[test]
fn smsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 32], OperandSize::Qword)
}

#[test]
fn smsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(Direct(RBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 1, 227], OperandSize::Qword)
}

#[test]
fn smsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SMSW, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 963111596, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 164, 130, 172, 234, 103, 57], OperandSize::Qword)
}

