use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovns_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 223], OperandSize::Word)
}

#[test]
fn cmovns_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(SI, 13383, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 140, 71, 52], OperandSize::Word)
}

#[test]
fn cmovns_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 230], OperandSize::Dword)
}

#[test]
fn cmovns_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 26507251, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 52, 93, 243, 119, 148, 1], OperandSize::Dword)
}

#[test]
fn cmovns_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 243], OperandSize::Qword)
}

#[test]
fn cmovns_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1239312008, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 36, 141, 136, 102, 222, 73], OperandSize::Qword)
}

#[test]
fn cmovns_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 228], OperandSize::Word)
}

#[test]
fn cmovns_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESI)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 52], OperandSize::Word)
}

#[test]
fn cmovns_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 222], OperandSize::Dword)
}

#[test]
fn cmovns_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 60, 67], OperandSize::Dword)
}

#[test]
fn cmovns_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 255], OperandSize::Qword)
}

#[test]
fn cmovns_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 39], OperandSize::Qword)
}

#[test]
fn cmovns_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 73, 219], OperandSize::Qword)
}

#[test]
fn cmovns_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RCX, 708960704, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 73, 145, 192, 225, 65, 42], OperandSize::Qword)
}

