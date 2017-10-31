use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 207], OperandSize::Word)
}

#[test]
fn cmovnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(DI, 13836, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 165, 12, 54], OperandSize::Word)
}

#[test]
fn cmovnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 204], OperandSize::Dword)
}

#[test]
fn cmovnb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1011466035, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 44, 149, 51, 191, 73, 60], OperandSize::Dword)
}

#[test]
fn cmovnb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 214], OperandSize::Qword)
}

#[test]
fn cmovnb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 2016359931, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 180, 94, 251, 53, 47, 120], OperandSize::Qword)
}

#[test]
fn cmovnb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 229], OperandSize::Word)
}

#[test]
fn cmovnb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(BP, 46, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 78, 46], OperandSize::Word)
}

#[test]
fn cmovnb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 220], OperandSize::Dword)
}

#[test]
fn cmovnb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1073502558, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 36, 245, 94, 89, 252, 63], OperandSize::Dword)
}

#[test]
fn cmovnb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 246], OperandSize::Qword)
}

#[test]
fn cmovnb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 23], OperandSize::Qword)
}

#[test]
fn cmovnb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 210], OperandSize::Qword)
}

#[test]
fn cmovnb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 1696041336, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 188, 214, 120, 137, 23, 101], OperandSize::Qword)
}

