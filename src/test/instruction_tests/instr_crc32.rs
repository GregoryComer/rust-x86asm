use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn crc32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 234], OperandSize::Word)
}

#[test]
fn crc32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 28], OperandSize::Word)
}

#[test]
fn crc32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 242], OperandSize::Dword)
}

#[test]
fn crc32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(EDX, 1141162610, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 186, 114, 194, 4, 68], OperandSize::Dword)
}

#[test]
fn crc32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 226], OperandSize::Qword)
}

#[test]
fn crc32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1698403449, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 20, 157, 121, 148, 59, 101], OperandSize::Qword)
}

#[test]
fn crc32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDX)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 211], OperandSize::Qword)
}

#[test]
fn crc32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 258492340, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 36, 141, 180, 71, 104, 15], OperandSize::Qword)
}

#[test]
fn crc32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RSP)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 240, 226], OperandSize::Qword)
}

#[test]
fn crc32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RDX)), operand2: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 240, 17], OperandSize::Qword)
}

#[test]
fn crc32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 249], OperandSize::Word)
}

#[test]
fn crc32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Indirect(SI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 36], OperandSize::Word)
}

#[test]
fn crc32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 233], OperandSize::Dword)
}

#[test]
fn crc32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1664161485, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 188, 190, 205, 22, 49, 99], OperandSize::Dword)
}

#[test]
fn crc32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 249], OperandSize::Qword)
}

#[test]
fn crc32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1334883741, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 36, 133, 157, 181, 144, 79], OperandSize::Qword)
}

#[test]
fn crc32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 239], OperandSize::Word)
}

#[test]
fn crc32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 59], OperandSize::Word)
}

#[test]
fn crc32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 217], OperandSize::Dword)
}

#[test]
fn crc32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 44, 80], OperandSize::Dword)
}

#[test]
fn crc32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 246], OperandSize::Qword)
}

#[test]
fn crc32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 44, 89], OperandSize::Qword)
}

#[test]
fn crc32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 241, 203], OperandSize::Qword)
}

#[test]
fn crc32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 1291622064, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 241, 164, 207, 176, 150, 252, 76], OperandSize::Qword)
}

