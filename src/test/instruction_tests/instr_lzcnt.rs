use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lzcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 223], OperandSize::Word)
}

#[test]
fn lzcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(DI, 90, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 101, 90], OperandSize::Word)
}

#[test]
fn lzcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 212], OperandSize::Dword)
}

#[test]
fn lzcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(EAX, 1191213984, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 144, 160, 123, 0, 71], OperandSize::Dword)
}

#[test]
fn lzcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 234], OperandSize::Qword)
}

#[test]
fn lzcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 446067752, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 60, 85, 40, 116, 150, 26], OperandSize::Qword)
}

#[test]
fn lzcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 225], OperandSize::Word)
}

#[test]
fn lzcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 18], OperandSize::Word)
}

#[test]
fn lzcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 249], OperandSize::Dword)
}

#[test]
fn lzcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 36, 145], OperandSize::Dword)
}

#[test]
fn lzcnt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 239], OperandSize::Qword)
}

#[test]
fn lzcnt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EDI)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 63], OperandSize::Qword)
}

#[test]
fn lzcnt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 189, 206], OperandSize::Qword)
}

#[test]
fn lzcnt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1156006010, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 189, 20, 245, 122, 64, 231, 68], OperandSize::Qword)
}

