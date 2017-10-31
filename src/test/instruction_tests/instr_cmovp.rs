use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 246], OperandSize::Word)
}

#[test]
fn cmovp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 207, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 168, 207, 0], OperandSize::Word)
}

#[test]
fn cmovp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 233], OperandSize::Dword)
}

#[test]
fn cmovp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(BX)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 24], OperandSize::Dword)
}

#[test]
fn cmovp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 244], OperandSize::Qword)
}

#[test]
fn cmovp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(RSI, 642244196, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 166, 100, 222, 71, 38], OperandSize::Qword)
}

#[test]
fn cmovp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 221], OperandSize::Word)
}

#[test]
fn cmovp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(BX, 30236, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 175, 28, 118], OperandSize::Word)
}

#[test]
fn cmovp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 209], OperandSize::Dword)
}

#[test]
fn cmovp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EDI)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 62], OperandSize::Dword)
}

#[test]
fn cmovp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 209], OperandSize::Qword)
}

#[test]
fn cmovp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RCX, 789521187, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 185, 35, 35, 15, 47], OperandSize::Qword)
}

#[test]
fn cmovp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 236], OperandSize::Qword)
}

#[test]
fn cmovp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 28, 159], OperandSize::Qword)
}

