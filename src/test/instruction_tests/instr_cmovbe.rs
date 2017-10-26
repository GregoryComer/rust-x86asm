use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 221], OperandSize::Word)
}

#[test]
fn cmovbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(SI, 20245, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 148, 21, 79], OperandSize::Word)
}

#[test]
fn cmovbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 219], OperandSize::Dword)
}

#[test]
fn cmovbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 12, 112], OperandSize::Dword)
}

#[test]
fn cmovbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 254], OperandSize::Qword)
}

#[test]
fn cmovbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 20, 152], OperandSize::Qword)
}

#[test]
fn cmovbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 201], OperandSize::Word)
}

#[test]
fn cmovbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(BX, 18, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 111, 18], OperandSize::Word)
}

#[test]
fn cmovbe_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 249], OperandSize::Dword)
}

#[test]
fn cmovbe_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 464604744, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 156, 154, 72, 78, 177, 27], OperandSize::Dword)
}

#[test]
fn cmovbe_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 243], OperandSize::Qword)
}

#[test]
fn cmovbe_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 34], OperandSize::Qword)
}

#[test]
fn cmovbe_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 242], OperandSize::Qword)
}

#[test]
fn cmovbe_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVBE, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RAX, 868591562, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 136, 202, 167, 197, 51], OperandSize::Qword)
}

