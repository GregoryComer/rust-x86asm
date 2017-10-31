use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 221], OperandSize::Word)
}

#[test]
fn cmovnp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 234, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 168, 234, 0], OperandSize::Word)
}

#[test]
fn cmovnp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 227], OperandSize::Dword)
}

#[test]
fn cmovnp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(EDX, 1023084189, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 146, 157, 6, 251, 60], OperandSize::Dword)
}

#[test]
fn cmovnp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 234], OperandSize::Qword)
}

#[test]
fn cmovnp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(RDI, 875631927, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 151, 55, 21, 49, 52], OperandSize::Qword)
}

#[test]
fn cmovnp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 202], OperandSize::Word)
}

#[test]
fn cmovnp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ESI)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 55], OperandSize::Word)
}

#[test]
fn cmovnp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 251], OperandSize::Dword)
}

#[test]
fn cmovnp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EDX)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 22], OperandSize::Dword)
}

#[test]
fn cmovnp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 247], OperandSize::Qword)
}

#[test]
fn cmovnp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 372644180, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 28, 245, 84, 25, 54, 22], OperandSize::Qword)
}

#[test]
fn cmovnp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 75, 214], OperandSize::Qword)
}

#[test]
fn cmovnp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RDX, 193208332, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 75, 138, 12, 32, 132, 11], OperandSize::Qword)
}

