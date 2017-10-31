use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 210], OperandSize::Word)
}

#[test]
fn cmovle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 11], OperandSize::Word)
}

#[test]
fn cmovle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 205], OperandSize::Dword)
}

#[test]
fn cmovle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 602736814, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 164, 134, 174, 8, 237, 35], OperandSize::Dword)
}

#[test]
fn cmovle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 205], OperandSize::Qword)
}

#[test]
fn cmovle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1437223111, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 172, 184, 199, 72, 170, 85], OperandSize::Qword)
}

#[test]
fn cmovle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 254], OperandSize::Word)
}

#[test]
fn cmovle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(EBP)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 47], OperandSize::Word)
}

#[test]
fn cmovle_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 231], OperandSize::Dword)
}

#[test]
fn cmovle_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 60, 83], OperandSize::Dword)
}

#[test]
fn cmovle_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 247], OperandSize::Qword)
}

#[test]
fn cmovle_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 520554558, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 140, 254, 62, 8, 7, 31], OperandSize::Qword)
}

#[test]
fn cmovle_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 254], OperandSize::Qword)
}

#[test]
fn cmovle_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 1760197629, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 156, 243, 253, 123, 234, 104], OperandSize::Qword)
}

