use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 4359, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 163, 7, 17], OperandSize::Word)
}

#[test]
fn movbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 60, 80], OperandSize::Dword)
}

#[test]
fn movbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1060345173, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 60, 85, 85, 149, 51, 63], OperandSize::Qword)
}

#[test]
fn movbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(DI, 27911, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 240, 157, 7, 109], OperandSize::Word)
}

#[test]
fn movbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 52, 222], OperandSize::Dword)
}

#[test]
fn movbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RSI, 1156606733, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 240, 182, 13, 107, 240, 68], OperandSize::Qword)
}

#[test]
fn movbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 56, 240, 46], OperandSize::Qword)
}

#[test]
fn movbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 11], OperandSize::Word)
}

#[test]
fn movbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1970102605, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 156, 86, 77, 97, 109, 117], OperandSize::Dword)
}

#[test]
fn movbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 63], OperandSize::Qword)
}

#[test]
fn movbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 241, 36], OperandSize::Word)
}

#[test]
fn movbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectDisplaced(EBX, 1665961668, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 139, 196, 142, 76, 99], OperandSize::Dword)
}

#[test]
fn movbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 564632270, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 241, 172, 64, 206, 154, 167, 33], OperandSize::Qword)
}

#[test]
fn movbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVBE, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1480347941, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 56, 241, 20, 197, 37, 81, 60, 88], OperandSize::Qword)
}

