use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movzx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(SI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 243], OperandSize::Word)
}

#[test]
fn movzx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(SI, 188, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 140, 188, 0], OperandSize::Word)
}

#[test]
fn movzx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(SP)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 227], OperandSize::Dword)
}

#[test]
fn movzx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 12, 184], OperandSize::Dword)
}

#[test]
fn movzx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(CX)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 202], OperandSize::Qword)
}

#[test]
fn movzx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 44, 134], OperandSize::Qword)
}

#[test]
fn movzx_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 241], OperandSize::Word)
}

#[test]
fn movzx_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 99, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 107, 99], OperandSize::Word)
}

#[test]
fn movzx_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EDI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 251], OperandSize::Dword)
}

#[test]
fn movzx_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(ESI, 115286077, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 166, 61, 32, 223, 6], OperandSize::Dword)
}

#[test]
fn movzx_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ECX)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 202], OperandSize::Qword)
}

#[test]
fn movzx_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 92031068, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 156, 73, 92, 72, 124, 5], OperandSize::Qword)
}

#[test]
fn movzx_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RSI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 182, 242], OperandSize::Qword)
}

#[test]
fn movzx_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 182, 46], OperandSize::Qword)
}

#[test]
fn movzx_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 183, 246], OperandSize::Word)
}

#[test]
fn movzx_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(SI, 7089, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 183, 148, 177, 27], OperandSize::Word)
}

#[test]
fn movzx_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 220], OperandSize::Dword)
}

#[test]
fn movzx_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESI)), operand2: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 51], OperandSize::Dword)
}

#[test]
fn movzx_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 237], OperandSize::Qword)
}

#[test]
fn movzx_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1931127633, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 28, 245, 81, 171, 26, 115], OperandSize::Qword)
}

#[test]
fn movzx_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RDX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 183, 210], OperandSize::Qword)
}

#[test]
fn movzx_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1516662895, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 183, 12, 245, 111, 112, 102, 90], OperandSize::Qword)
}

