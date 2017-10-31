use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 202], OperandSize::Word)
}

#[test]
fn cmovnl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 27271, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 162, 135, 106], OperandSize::Word)
}

#[test]
fn cmovnl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 210], OperandSize::Dword)
}

#[test]
fn cmovnl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 52, 78], OperandSize::Dword)
}

#[test]
fn cmovnl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 255], OperandSize::Qword)
}

#[test]
fn cmovnl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(DI)), operand2: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 63], OperandSize::Qword)
}

#[test]
fn cmovnl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 246], OperandSize::Word)
}

#[test]
fn cmovnl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(BX, 86, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 87, 86], OperandSize::Word)
}

#[test]
fn cmovnl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 243], OperandSize::Dword)
}

#[test]
fn cmovnl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(EDI, 1162832215, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 151, 87, 105, 79, 69], OperandSize::Dword)
}

#[test]
fn cmovnl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 227], OperandSize::Qword)
}

#[test]
fn cmovnl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1082187968, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 44, 117, 192, 224, 128, 64], OperandSize::Qword)
}

#[test]
fn cmovnl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 234], OperandSize::Qword)
}

#[test]
fn cmovnl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 40843758, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 52, 133, 238, 57, 111, 2], OperandSize::Qword)
}

