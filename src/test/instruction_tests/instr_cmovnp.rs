use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 222], OperandSize::Word)
}

#[test]
fn cmovnp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 178, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 171, 178, 0], OperandSize::Word)
}

#[test]
fn cmovnp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 254], OperandSize::Dword)
}

#[test]
fn cmovnp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SI)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 50], OperandSize::Dword)
}

#[test]
fn cmovnp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 231], OperandSize::Qword)
}

#[test]
fn cmovnp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(CX)), operand2: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 14], OperandSize::Qword)
}

#[test]
fn cmovnp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 210], OperandSize::Word)
}

#[test]
fn cmovnp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 80, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 99, 80], OperandSize::Word)
}

#[test]
fn cmovnp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 210], OperandSize::Dword)
}

#[test]
fn cmovnp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 857358285, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 12, 85, 205, 63, 26, 51], OperandSize::Dword)
}

#[test]
fn cmovnp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 217], OperandSize::Qword)
}

#[test]
fn cmovnp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 506909245, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 140, 142, 61, 210, 54, 30], OperandSize::Qword)
}

#[test]
fn cmovnp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 75, 218], OperandSize::Qword)
}

#[test]
fn cmovnp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RBX, 1619990216, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 75, 139, 200, 22, 143, 96], OperandSize::Qword)
}

