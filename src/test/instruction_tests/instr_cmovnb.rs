use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 220], OperandSize::Word)
}

#[test]
fn cmovnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(BX, 31085, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 183, 109, 121], OperandSize::Word)
}

#[test]
fn cmovnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 252], OperandSize::Dword)
}

#[test]
fn cmovnb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 610419977, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 164, 71, 9, 69, 98, 36], OperandSize::Dword)
}

#[test]
fn cmovnb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 202], OperandSize::Qword)
}

#[test]
fn cmovnb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1893173944, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 36, 157, 184, 138, 215, 112], OperandSize::Qword)
}

#[test]
fn cmovnb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 233], OperandSize::Word)
}

#[test]
fn cmovnb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 56, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 114, 56], OperandSize::Word)
}

#[test]
fn cmovnb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 255], OperandSize::Dword)
}

#[test]
fn cmovnb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 402387101, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 20, 133, 157, 240, 251, 23], OperandSize::Dword)
}

#[test]
fn cmovnb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 243], OperandSize::Qword)
}

#[test]
fn cmovnb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 44, 177], OperandSize::Qword)
}

#[test]
fn cmovnb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 213], OperandSize::Qword)
}

#[test]
fn cmovnb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1736326313, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 28, 253, 169, 60, 126, 103], OperandSize::Qword)
}

