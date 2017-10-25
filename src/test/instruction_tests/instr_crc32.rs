use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn crc32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 209], OperandSize::Word)
}

#[test]
fn crc32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(BP, 2289, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 190, 241, 8], OperandSize::Word)
}

#[test]
fn crc32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 235], OperandSize::Dword)
}

#[test]
fn crc32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1127314869, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 20, 69, 181, 117, 49, 67], OperandSize::Dword)
}

#[test]
fn crc32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 251], OperandSize::Qword)
}

#[test]
fn crc32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 548448033, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 148, 74, 33, 167, 176, 32], OperandSize::Qword)
}

#[test]
fn crc32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 219], OperandSize::Qword)
}

#[test]
fn crc32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RAX, 158718835, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 184, 115, 219, 117, 9], OperandSize::Qword)
}

#[test]
fn crc32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RSI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 240, 241], OperandSize::Qword)
}

#[test]
fn crc32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 743771965, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 240, 148, 207, 61, 15, 85, 44], OperandSize::Qword)
}

#[test]
fn crc32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 234], OperandSize::Word)
}

#[test]
fn crc32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 9], OperandSize::Word)
}

#[test]
fn crc32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 241], OperandSize::Dword)
}

#[test]
fn crc32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 517777993, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 188, 130, 73, 170, 220, 30], OperandSize::Dword)
}

#[test]
fn crc32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 234], OperandSize::Qword)
}

#[test]
fn crc32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1465932451, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 36, 149, 163, 90, 96, 87], OperandSize::Qword)
}

#[test]
fn crc32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 246], OperandSize::Word)
}

#[test]
fn crc32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 51], OperandSize::Word)
}

#[test]
fn crc32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 226], OperandSize::Dword)
}

#[test]
fn crc32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 853258303, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 60, 125, 63, 176, 219, 50], OperandSize::Dword)
}

#[test]
fn crc32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 229], OperandSize::Qword)
}

#[test]
fn crc32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 851568380, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 188, 136, 252, 230, 193, 50], OperandSize::Qword)
}

#[test]
fn crc32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 241, 227], OperandSize::Qword)
}

#[test]
fn crc32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 241, 12, 250], OperandSize::Qword)
}

