use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovna_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 244], OperandSize::Word)
}

#[test]
fn cmovna_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(SI, 28985, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 180, 57, 113], OperandSize::Word)
}

#[test]
fn cmovna_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 209], OperandSize::Dword)
}

#[test]
fn cmovna_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(BX)), operand2: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 31], OperandSize::Dword)
}

#[test]
fn cmovna_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 220], OperandSize::Qword)
}

#[test]
fn cmovna_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(CX)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 11], OperandSize::Qword)
}

#[test]
fn cmovna_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 214], OperandSize::Word)
}

#[test]
fn cmovna_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 12428, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 168, 140, 48], OperandSize::Word)
}

#[test]
fn cmovna_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 237], OperandSize::Dword)
}

#[test]
fn cmovna_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EAX, 1209490376, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 168, 200, 91, 23, 72], OperandSize::Dword)
}

#[test]
fn cmovna_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 217], OperandSize::Qword)
}

#[test]
fn cmovna_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 439248315, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 172, 184, 187, 101, 46, 26], OperandSize::Qword)
}

#[test]
fn cmovna_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 219], OperandSize::Qword)
}

#[test]
fn cmovna_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 41], OperandSize::Qword)
}

