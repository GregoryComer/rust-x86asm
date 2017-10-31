use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovng_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 230], OperandSize::Word)
}

#[test]
fn cmovng_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(DI, 7301, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 189, 133, 28], OperandSize::Word)
}

#[test]
fn cmovng_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 247], OperandSize::Dword)
}

#[test]
fn cmovng_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 256081125, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 28, 93, 229, 124, 67, 15], OperandSize::Dword)
}

#[test]
fn cmovng_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 225], OperandSize::Qword)
}

#[test]
fn cmovng_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(SP)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 32], OperandSize::Qword)
}

#[test]
fn cmovng_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 203], OperandSize::Word)
}

#[test]
fn cmovng_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(BP, 176, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 150, 176, 0], OperandSize::Word)
}

#[test]
fn cmovng_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 212], OperandSize::Dword)
}

#[test]
fn cmovng_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 100345501, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 140, 115, 157, 38, 251, 5], OperandSize::Dword)
}

#[test]
fn cmovng_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 227], OperandSize::Qword)
}

#[test]
fn cmovng_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(RSI, 1377291897, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 174, 121, 206, 23, 82], OperandSize::Qword)
}

#[test]
fn cmovng_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 206], OperandSize::Qword)
}

#[test]
fn cmovng_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1802332836, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 156, 95, 164, 106, 109, 107], OperandSize::Qword)
}

