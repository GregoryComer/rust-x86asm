use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovng_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 226], OperandSize::Word)
}

#[test]
fn cmovng_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(BP)), operand2: Some(Indirect(SI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 44], OperandSize::Word)
}

#[test]
fn cmovng_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 217], OperandSize::Dword)
}

#[test]
fn cmovng_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 44, 216], OperandSize::Dword)
}

#[test]
fn cmovng_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 250], OperandSize::Qword)
}

#[test]
fn cmovng_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 1580389018, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 172, 137, 154, 210, 50, 94], OperandSize::Qword)
}

#[test]
fn cmovng_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 213], OperandSize::Word)
}

#[test]
fn cmovng_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(BP, 29203, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 182, 19, 114], OperandSize::Word)
}

#[test]
fn cmovng_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 246], OperandSize::Dword)
}

#[test]
fn cmovng_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EDI)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 58], OperandSize::Dword)
}

#[test]
fn cmovng_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 215], OperandSize::Qword)
}

#[test]
fn cmovng_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 43], OperandSize::Qword)
}

#[test]
fn cmovng_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 214], OperandSize::Qword)
}

#[test]
fn cmovng_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 534064749, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 172, 191, 109, 46, 213, 31], OperandSize::Qword)
}

