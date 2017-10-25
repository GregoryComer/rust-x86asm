use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 225], OperandSize::Word)
}

#[test]
fn cmovnc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 3315, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 176, 243, 12], OperandSize::Word)
}

#[test]
fn cmovnc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 243], OperandSize::Dword)
}

#[test]
fn cmovnc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 184133298, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 188, 216, 178, 166, 249, 10], OperandSize::Dword)
}

#[test]
fn cmovnc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 214], OperandSize::Qword)
}

#[test]
fn cmovnc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 28, 67], OperandSize::Qword)
}

#[test]
fn cmovnc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 229], OperandSize::Word)
}

#[test]
fn cmovnc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(BX, 80, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 127, 80], OperandSize::Word)
}

#[test]
fn cmovnc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 221], OperandSize::Dword)
}

#[test]
fn cmovnc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 2072298277, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 36, 157, 37, 195, 132, 123], OperandSize::Dword)
}

#[test]
fn cmovnc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 227], OperandSize::Qword)
}

#[test]
fn cmovnc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 35], OperandSize::Qword)
}

#[test]
fn cmovnc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 230], OperandSize::Qword)
}

#[test]
fn cmovnc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 28, 123], OperandSize::Qword)
}

