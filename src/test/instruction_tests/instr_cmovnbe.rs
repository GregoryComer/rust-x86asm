use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 252], OperandSize::Word)
}

#[test]
fn cmovnbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BX)), operand2: Some(Memory(19474, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 30, 18, 76], OperandSize::Word)
}

#[test]
fn cmovnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 230], OperandSize::Dword)
}

#[test]
fn cmovnbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 36, 194], OperandSize::Dword)
}

#[test]
fn cmovnbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 239], OperandSize::Qword)
}

#[test]
fn cmovnbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 162370960, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 44, 133, 144, 149, 173, 9], OperandSize::Qword)
}

#[test]
fn cmovnbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 202], OperandSize::Word)
}

#[test]
fn cmovnbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 96, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 71, 107, 96], OperandSize::Word)
}

#[test]
fn cmovnbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 213], OperandSize::Dword)
}

#[test]
fn cmovnbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1061148216, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 52, 149, 56, 214, 63, 63], OperandSize::Dword)
}

#[test]
fn cmovnbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 226], OperandSize::Qword)
}

#[test]
fn cmovnbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 2114431587, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 71, 12, 181, 99, 170, 7, 126], OperandSize::Qword)
}

#[test]
fn cmovnbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 205], OperandSize::Qword)
}

#[test]
fn cmovnbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNBE, operand1: Some(Direct(RSP)), operand2: Some(IndirectDisplaced(RAX, 1047583686, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 71, 160, 198, 219, 112, 62], OperandSize::Qword)
}

