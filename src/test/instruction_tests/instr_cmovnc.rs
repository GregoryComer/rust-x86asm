use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 204], OperandSize::Word)
}

#[test]
fn cmovnc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(SI, 21739, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 140, 235, 84], OperandSize::Word)
}

#[test]
fn cmovnc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 219], OperandSize::Dword)
}

#[test]
fn cmovnc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(EDX, 464346760, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 170, 136, 94, 173, 27], OperandSize::Dword)
}

#[test]
fn cmovnc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 221], OperandSize::Qword)
}

#[test]
fn cmovnc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 12, 73], OperandSize::Qword)
}

#[test]
fn cmovnc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 202], OperandSize::Word)
}

#[test]
fn cmovnc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EBP)), operand2: Some(Memory(23886, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 46, 78, 93], OperandSize::Word)
}

#[test]
fn cmovnc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 221], OperandSize::Dword)
}

#[test]
fn cmovnc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EDI)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 59], OperandSize::Dword)
}

#[test]
fn cmovnc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 220], OperandSize::Qword)
}

#[test]
fn cmovnc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RAX, 520649254, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 184, 38, 122, 8, 31], OperandSize::Qword)
}

#[test]
fn cmovnc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(RSI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 241], OperandSize::Qword)
}

#[test]
fn cmovnc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 201554398, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 164, 182, 222, 121, 3, 12], OperandSize::Qword)
}

