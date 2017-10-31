use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn crc32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ECX)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 202], OperandSize::Word)
}

#[test]
fn crc32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(BX, 17844, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 159, 180, 69], OperandSize::Word)
}

#[test]
fn crc32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 250], OperandSize::Dword)
}

#[test]
fn crc32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EAX, 1315494088, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 152, 200, 216, 104, 78], OperandSize::Dword)
}

#[test]
fn crc32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 243], OperandSize::Qword)
}

#[test]
fn crc32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RSI, 2036226931, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 158, 115, 91, 94, 121], OperandSize::Qword)
}

#[test]
fn crc32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 226], OperandSize::Qword)
}

#[test]
fn crc32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(RBX, 1430030239, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 139, 159, 135, 60, 85], OperandSize::Qword)
}

#[test]
fn crc32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RDI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 240, 251], OperandSize::Qword)
}

#[test]
fn crc32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RCX)), operand2: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 240, 8], OperandSize::Qword)
}

#[test]
fn crc32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 227], OperandSize::Word)
}

#[test]
fn crc32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 32], OperandSize::Word)
}

#[test]
fn crc32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 235], OperandSize::Dword)
}

#[test]
fn crc32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 60, 122], OperandSize::Dword)
}

#[test]
fn crc32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 255], OperandSize::Qword)
}

#[test]
fn crc32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 4052386, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 60, 245, 162, 213, 61, 0], OperandSize::Qword)
}

#[test]
fn crc32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 241], OperandSize::Word)
}

#[test]
fn crc32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 32], OperandSize::Word)
}

#[test]
fn crc32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 217], OperandSize::Dword)
}

#[test]
fn crc32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(EBX, 1097443477, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 179, 149, 168, 105, 65], OperandSize::Dword)
}

#[test]
fn crc32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 221], OperandSize::Qword)
}

#[test]
fn crc32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 581398362, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 180, 114, 90, 111, 167, 34], OperandSize::Qword)
}

#[test]
fn crc32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 241, 227], OperandSize::Qword)
}

#[test]
fn crc32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1346677315, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 241, 44, 197, 67, 170, 68, 80], OperandSize::Qword)
}

