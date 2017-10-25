use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn popcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 237], OperandSize::Word)
}

#[test]
fn popcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(SI, 56, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 116, 56], OperandSize::Word)
}

#[test]
fn popcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 237], OperandSize::Dword)
}

#[test]
fn popcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 52, 215], OperandSize::Dword)
}

#[test]
fn popcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 228], OperandSize::Qword)
}

#[test]
fn popcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 52, 120], OperandSize::Qword)
}

#[test]
fn popcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 214], OperandSize::Word)
}

#[test]
fn popcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(DI, 218, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 141, 218, 0], OperandSize::Word)
}

#[test]
fn popcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 201], OperandSize::Dword)
}

#[test]
fn popcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(ECX, 1062147929, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 153, 89, 23, 79, 63], OperandSize::Dword)
}

#[test]
fn popcnt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 249], OperandSize::Qword)
}

#[test]
fn popcnt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 558483749, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 60, 133, 37, 201, 73, 33], OperandSize::Qword)
}

#[test]
fn popcnt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 184, 223], OperandSize::Qword)
}

#[test]
fn popcnt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1042991705, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 184, 188, 134, 89, 202, 42, 62], OperandSize::Qword)
}

