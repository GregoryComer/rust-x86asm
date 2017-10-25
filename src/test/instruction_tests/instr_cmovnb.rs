use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 239], OperandSize::Word)
}

#[test]
fn cmovnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 14939, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 155, 91, 58], OperandSize::Word)
}

#[test]
fn cmovnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 230], OperandSize::Dword)
}

#[test]
fn cmovnb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(ESI, 568511714, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 158, 226, 204, 226, 33], OperandSize::Dword)
}

#[test]
fn cmovnb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 242], OperandSize::Qword)
}

#[test]
fn cmovnb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(SP)), operand2: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 38], OperandSize::Qword)
}

#[test]
fn cmovnb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 255], OperandSize::Word)
}

#[test]
fn cmovnb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(DI, 30720, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 181, 0, 120], OperandSize::Word)
}

#[test]
fn cmovnb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 247], OperandSize::Dword)
}

#[test]
fn cmovnb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 643579984, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 52, 197, 80, 64, 92, 38], OperandSize::Dword)
}

#[test]
fn cmovnb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 237], OperandSize::Qword)
}

#[test]
fn cmovnb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(RSI, 954052594, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 174, 242, 175, 221, 56], OperandSize::Qword)
}

#[test]
fn cmovnb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 226], OperandSize::Qword)
}

#[test]
fn cmovnb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNB, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 20, 144], OperandSize::Qword)
}

