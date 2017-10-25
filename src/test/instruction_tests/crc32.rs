use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn crc32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 251], OperandSize::Word)
}

fn crc32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 51], OperandSize::Word)
}

fn crc32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 241], OperandSize::Dword)
}

fn crc32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 810920877, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 188, 207, 173, 171, 85, 48], OperandSize::Dword)
}

fn crc32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 250], OperandSize::Qword)
}

fn crc32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(RBX, 700792045, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 171, 237, 60, 197, 41], OperandSize::Qword)
}

fn crc32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 250], OperandSize::Qword)
}

fn crc32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 266405248, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 240, 28, 125, 128, 5, 225, 15], OperandSize::Qword)
}

fn crc32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RBX)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 240, 218], OperandSize::Qword)
}

fn crc32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 1683758380, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 240, 156, 94, 44, 29, 92, 100], OperandSize::Qword)
}

fn crc32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 223], OperandSize::Word)
}

fn crc32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(BX, 57, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 119, 57], OperandSize::Word)
}

fn crc32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ECX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 201], OperandSize::Dword)
}

fn crc32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 28, 136], OperandSize::Dword)
}

fn crc32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ECX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 202], OperandSize::Qword)
}

fn crc32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RDX, 248132141, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 146, 45, 50, 202, 14], OperandSize::Qword)
}

fn crc32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 241], OperandSize::Word)
}

fn crc32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 118, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 242, 15, 56, 241, 97, 118], OperandSize::Word)
}

fn crc32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 228], OperandSize::Dword)
}

fn crc32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1325685406, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 148, 240, 158, 90, 4, 79], OperandSize::Dword)
}

fn crc32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 227], OperandSize::Qword)
}

fn crc32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 56, 241, 28, 183], OperandSize::Qword)
}

fn crc32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 241, 252], OperandSize::Qword)
}

fn crc32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::CRC32, operand1: Some(Direct(RSP)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 56, 241, 34], OperandSize::Qword)
}

