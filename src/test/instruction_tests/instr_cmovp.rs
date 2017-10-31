use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 226], OperandSize::Word)
}

#[test]
fn cmovp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 160, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 147, 160, 0], OperandSize::Word)
}

#[test]
fn cmovp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 234], OperandSize::Dword)
}

#[test]
fn cmovp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1206622666, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 36, 189, 202, 153, 235, 71], OperandSize::Dword)
}

#[test]
fn cmovp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 233], OperandSize::Qword)
}

#[test]
fn cmovp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1649763991, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 36, 205, 151, 102, 85, 98], OperandSize::Qword)
}

#[test]
fn cmovp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 218], OperandSize::Word)
}

#[test]
fn cmovp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ESI)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 52], OperandSize::Word)
}

#[test]
fn cmovp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 250], OperandSize::Dword)
}

#[test]
fn cmovp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EBX)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 26], OperandSize::Dword)
}

#[test]
fn cmovp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 228], OperandSize::Qword)
}

#[test]
fn cmovp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RDI, 1333672930, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 167, 226, 59, 126, 79], OperandSize::Qword)
}

#[test]
fn cmovp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(RSI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 241], OperandSize::Qword)
}

#[test]
fn cmovp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 62], OperandSize::Qword)
}

