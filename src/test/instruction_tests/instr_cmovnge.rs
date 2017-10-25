use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 214], OperandSize::Word)
}

#[test]
fn cmovnge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(SI, 5446, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 180, 70, 21], OperandSize::Word)
}

#[test]
fn cmovnge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 249], OperandSize::Dword)
}

#[test]
fn cmovnge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1367174961, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 164, 251, 49, 111, 125, 81], OperandSize::Dword)
}

#[test]
fn cmovnge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 241], OperandSize::Qword)
}

#[test]
fn cmovnge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(RAX, 2081751629, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 184, 77, 2, 21, 124], OperandSize::Qword)
}

#[test]
fn cmovnge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 206], OperandSize::Word)
}

#[test]
fn cmovnge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 76, 51], OperandSize::Word)
}

#[test]
fn cmovnge_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 211], OperandSize::Dword)
}

#[test]
fn cmovnge_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 20, 207], OperandSize::Dword)
}

#[test]
fn cmovnge_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 249], OperandSize::Qword)
}

#[test]
fn cmovnge_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 990983879, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 76, 12, 141, 199, 54, 17, 59], OperandSize::Qword)
}

#[test]
fn cmovnge_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 243], OperandSize::Qword)
}

#[test]
fn cmovnge_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNGE, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1199504921, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 76, 52, 77, 25, 254, 126, 71], OperandSize::Qword)
}

