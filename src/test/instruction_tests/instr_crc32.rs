use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn crc32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 227], OperandSize::Word)
}

#[test]
fn crc32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 50], OperandSize::Word)
}

#[test]
fn crc32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ECX)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 202], OperandSize::Dword)
}

#[test]
fn crc32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ECX)), operand2: Some(Indirect(EDX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 10], OperandSize::Dword)
}

#[test]
fn crc32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 249], OperandSize::Qword)
}

#[test]
fn crc32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 816564587, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 36, 133, 107, 201, 171, 48], OperandSize::Qword)
}

#[test]
fn crc32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 226], OperandSize::Qword)
}

#[test]
fn crc32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RDX, 1900925449, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 162, 9, 210, 77, 113], OperandSize::Qword)
}

#[test]
fn crc32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RSI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 240, 243], OperandSize::Qword)
}

#[test]
fn crc32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1698499120, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 240, 172, 75, 48, 10, 61, 101], OperandSize::Qword)
}

#[test]
fn crc32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 231], OperandSize::Word)
}

#[test]
fn crc32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 25], OperandSize::Word)
}

#[test]
fn crc32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 222], OperandSize::Dword)
}

#[test]
fn crc32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 673843497, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 52, 85, 41, 9, 42, 40], OperandSize::Dword)
}

#[test]
fn crc32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 255], OperandSize::Qword)
}

#[test]
fn crc32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 10], OperandSize::Qword)
}

#[test]
fn crc32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 223], OperandSize::Word)
}

#[test]
fn crc32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 29], OperandSize::Word)
}

#[test]
fn crc32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 217], OperandSize::Dword)
}

#[test]
fn crc32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 47], OperandSize::Dword)
}

#[test]
fn crc32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 239], OperandSize::Qword)
}

#[test]
fn crc32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1583398480, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 12, 85, 80, 190, 96, 94], OperandSize::Qword)
}

#[test]
fn crc32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 241, 220], OperandSize::Qword)
}

#[test]
fn crc32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 891296056, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 241, 148, 185, 56, 25, 32, 53], OperandSize::Qword)
}

