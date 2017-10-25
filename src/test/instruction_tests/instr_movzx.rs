use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movzx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 241], OperandSize::Word)
}

#[test]
fn movzx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(CX)), operand2: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 13], OperandSize::Word)
}

#[test]
fn movzx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 225], OperandSize::Dword)
}

#[test]
fn movzx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(BP)), operand2: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 43], OperandSize::Dword)
}

#[test]
fn movzx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(SP)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 226], OperandSize::Qword)
}

#[test]
fn movzx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 12, 120], OperandSize::Qword)
}

#[test]
fn movzx_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ECX)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 203], OperandSize::Word)
}

#[test]
fn movzx_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESI)), operand2: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 53], OperandSize::Word)
}

#[test]
fn movzx_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EDI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 250], OperandSize::Dword)
}

#[test]
fn movzx_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(EAX, 1799194258, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 160, 146, 134, 61, 107], OperandSize::Dword)
}

#[test]
fn movzx_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 241], OperandSize::Qword)
}

#[test]
fn movzx_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 623700156, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 140, 206, 188, 232, 44, 37], OperandSize::Qword)
}

#[test]
fn movzx_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RCX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 182, 201], OperandSize::Qword)
}

#[test]
fn movzx_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 900278037, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 182, 156, 200, 21, 39, 169, 53], OperandSize::Qword)
}

#[test]
fn movzx_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 183, 225], OperandSize::Word)
}

#[test]
fn movzx_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 25447, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 183, 171, 103, 99], OperandSize::Word)
}

#[test]
fn movzx_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 219], OperandSize::Dword)
}

#[test]
fn movzx_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 36, 145], OperandSize::Dword)
}

#[test]
fn movzx_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 226], OperandSize::Qword)
}

#[test]
fn movzx_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RAX, 961498949, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 152, 69, 79, 79, 57], OperandSize::Qword)
}

#[test]
fn movzx_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RBP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 183, 235], OperandSize::Qword)
}

#[test]
fn movzx_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 183, 44, 72], OperandSize::Qword)
}

