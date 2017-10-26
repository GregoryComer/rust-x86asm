use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 242], OperandSize::Word)
}

#[test]
fn cmovnl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(BX, 14756, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 143, 164, 57], OperandSize::Word)
}

#[test]
fn cmovnl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 219], OperandSize::Dword)
}

#[test]
fn cmovnl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(DI)), operand2: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 57], OperandSize::Dword)
}

#[test]
fn cmovnl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 254], OperandSize::Qword)
}

#[test]
fn cmovnl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 44, 135], OperandSize::Qword)
}

#[test]
fn cmovnl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 218], OperandSize::Word)
}

#[test]
fn cmovnl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(DI, 12672, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 181, 128, 49], OperandSize::Word)
}

#[test]
fn cmovnl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 207], OperandSize::Dword)
}

#[test]
fn cmovnl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(EAX, 1977090176, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 144, 128, 0, 216, 117], OperandSize::Dword)
}

#[test]
fn cmovnl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 226], OperandSize::Qword)
}

#[test]
fn cmovnl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1795951484, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 172, 88, 124, 11, 12, 107], OperandSize::Qword)
}

#[test]
fn cmovnl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 226], OperandSize::Qword)
}

#[test]
fn cmovnl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 397432278, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 188, 70, 214, 85, 176, 23], OperandSize::Qword)
}

