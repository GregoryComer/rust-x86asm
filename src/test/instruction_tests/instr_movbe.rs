use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 34], OperandSize::Word)
}

#[test]
fn movbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(CX)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 8], OperandSize::Dword)
}

#[test]
fn movbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(RDI, 944547928, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 143, 88, 168, 76, 56], OperandSize::Qword)
}

#[test]
fn movbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(BX, 178, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 175, 178, 0], OperandSize::Word)
}

#[test]
fn movbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(EBX)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 24], OperandSize::Dword)
}

#[test]
fn movbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 917567003, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 180, 91, 27, 246, 176, 54], OperandSize::Qword)
}

#[test]
fn movbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1696494424, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 56, 240, 148, 67, 88, 115, 30, 101], OperandSize::Qword)
}

#[test]
fn movbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectDisplaced(DI, 11590, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 149, 70, 45], OperandSize::Word)
}

#[test]
fn movbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 22], OperandSize::Dword)
}

#[test]
fn movbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 34], OperandSize::Qword)
}

#[test]
fn movbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 51], OperandSize::Word)
}

#[test]
fn movbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1736276699, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 164, 86, 219, 122, 125, 103], OperandSize::Dword)
}

#[test]
fn movbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 36, 145], OperandSize::Qword)
}

#[test]
fn movbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 56, 241, 20, 112], OperandSize::Qword)
}

