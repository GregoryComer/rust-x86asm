use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 253], OperandSize::Word)
}

#[test]
fn cmovle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(DI, 158, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 173, 158, 0], OperandSize::Word)
}

#[test]
fn cmovle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 212], OperandSize::Dword)
}

#[test]
fn cmovle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 928609246, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 156, 153, 222, 115, 89, 55], OperandSize::Dword)
}

#[test]
fn cmovle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 249], OperandSize::Qword)
}

#[test]
fn cmovle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(RDX, 692766169, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 170, 217, 197, 74, 41], OperandSize::Qword)
}

#[test]
fn cmovle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 235], OperandSize::Word)
}

#[test]
fn cmovle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESP)), operand2: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 37], OperandSize::Word)
}

#[test]
fn cmovle_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 247], OperandSize::Dword)
}

#[test]
fn cmovle_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 52, 192], OperandSize::Dword)
}

#[test]
fn cmovle_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 242], OperandSize::Qword)
}

#[test]
fn cmovle_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 12, 72], OperandSize::Qword)
}

#[test]
fn cmovle_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 255], OperandSize::Qword)
}

#[test]
fn cmovle_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(RSI)), operand2: Some(IndirectDisplaced(RCX, 1295538674, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 177, 242, 89, 56, 77], OperandSize::Qword)
}

