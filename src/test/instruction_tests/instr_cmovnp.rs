use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 233], OperandSize::Word)
}

#[test]
fn cmovnp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 30, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 89, 30], OperandSize::Word)
}

#[test]
fn cmovnp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 225], OperandSize::Dword)
}

#[test]
fn cmovnp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1656626052, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 52, 181, 132, 27, 190, 98], OperandSize::Dword)
}

#[test]
fn cmovnp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 247], OperandSize::Qword)
}

#[test]
fn cmovnp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 637051544, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 52, 221, 152, 162, 248, 37], OperandSize::Qword)
}

#[test]
fn cmovnp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 236], OperandSize::Word)
}

#[test]
fn cmovnp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 2340, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 163, 36, 9], OperandSize::Word)
}

#[test]
fn cmovnp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 242], OperandSize::Dword)
}

#[test]
fn cmovnp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1577851175, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 12, 157, 39, 25, 12, 94], OperandSize::Dword)
}

#[test]
fn cmovnp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 201], OperandSize::Qword)
}

#[test]
fn cmovnp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 60, 153], OperandSize::Qword)
}

#[test]
fn cmovnp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 75, 211], OperandSize::Qword)
}

#[test]
fn cmovnp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1042947618, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 75, 28, 157, 34, 30, 42, 62], OperandSize::Qword)
}

