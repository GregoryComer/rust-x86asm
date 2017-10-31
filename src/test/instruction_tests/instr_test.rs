use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn test_1() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 217], OperandSize::Word)
}

#[test]
fn test_2() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 8], OperandSize::Word)
}

#[test]
fn test_3() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 217], OperandSize::Dword)
}

#[test]
fn test_4() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 15], OperandSize::Dword)
}

#[test]
fn test_5() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 218], OperandSize::Qword)
}

#[test]
fn test_6() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 23], OperandSize::Qword)
}

#[test]
fn test_7() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 210], OperandSize::Qword)
}

#[test]
fn test_8() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 20, 240], OperandSize::Qword)
}

#[test]
fn test_9() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 202], OperandSize::Word)
}

#[test]
fn test_10() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(BP, 169, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 174, 169, 0], OperandSize::Word)
}

#[test]
fn test_11() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 201], OperandSize::Dword)
}

#[test]
fn test_12() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 62], OperandSize::Dword)
}

#[test]
fn test_13() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 253], OperandSize::Qword)
}

#[test]
fn test_14() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(RDI, 867115333, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 175, 69, 33, 175, 51], OperandSize::Qword)
}

#[test]
fn test_15() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 250], OperandSize::Word)
}

#[test]
fn test_16() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(SI, 835, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 172, 67, 3], OperandSize::Word)
}

#[test]
fn test_17() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 202], OperandSize::Dword)
}

#[test]
fn test_18() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1925460470, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 28, 117, 246, 49, 196, 114], OperandSize::Dword)
}

#[test]
fn test_19() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 207], OperandSize::Qword)
}

#[test]
fn test_20() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 52, 154], OperandSize::Qword)
}

#[test]
fn test_21() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 133, 251], OperandSize::Qword)
}

#[test]
fn test_22() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 910254842, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 133, 12, 213, 250, 98, 65, 54], OperandSize::Qword)
}

#[test]
fn test_23() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 99], OperandSize::Word)
}

#[test]
fn test_24() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 30], OperandSize::Dword)
}

#[test]
fn test_25() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 22], OperandSize::Qword)
}

#[test]
fn test_26() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(25266)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 178, 98], OperandSize::Word)
}

#[test]
fn test_27() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(6058)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 170, 23], OperandSize::Dword)
}

#[test]
fn test_28() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(30470)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 6, 119], OperandSize::Qword)
}

#[test]
fn test_29() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1290244241)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 145, 144, 231, 76], OperandSize::Word)
}

#[test]
fn test_30() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1280215063)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 23, 136, 78, 76], OperandSize::Dword)
}

#[test]
fn test_31() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(974932272)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 48, 73, 28, 58], OperandSize::Qword)
}

#[test]
fn test_32() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1384231353)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 169, 185, 177, 129, 82], OperandSize::Qword)
}

#[test]
fn test_33() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 193, 73], OperandSize::Word)
}

#[test]
fn test_34() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 1, 74], OperandSize::Word)
}

#[test]
fn test_35() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 194, 58], OperandSize::Dword)
}

#[test]
fn test_36() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(EBX, Two, 2121654409, Some(OperandSize::Byte), None)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 4, 93, 137, 224, 117, 126, 120], OperandSize::Dword)
}

#[test]
fn test_37() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BL)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 195, 63], OperandSize::Qword)
}

#[test]
fn test_38() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 407401299, Some(OperandSize::Byte), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 132, 64, 83, 115, 72, 24, 66], OperandSize::Qword)
}

#[test]
fn test_39() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 194, 2], OperandSize::Qword)
}

#[test]
fn test_40() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(RBX, 1028117758, Some(OperandSize::Byte), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 131, 254, 212, 71, 61, 71], OperandSize::Qword)
}

#[test]
fn test_41() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DI)), operand2: Some(Literal16(13952)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 199, 128, 54], OperandSize::Word)
}

#[test]
fn test_42() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(BP, 14021, Some(OperandSize::Word), None)), operand2: Some(Literal16(30441)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 134, 197, 54, 233, 118], OperandSize::Word)
}

#[test]
fn test_43() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CX)), operand2: Some(Literal16(16636)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 193, 252, 64], OperandSize::Dword)
}

#[test]
fn test_44() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1791427806, Some(OperandSize::Word), None)), operand2: Some(Literal16(6184)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 132, 215, 222, 4, 199, 106, 40, 24], OperandSize::Dword)
}

#[test]
fn test_45() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DX)), operand2: Some(Literal16(5744)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 194, 112, 22], OperandSize::Qword)
}

#[test]
fn test_46() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(RSI, 1746021988, Some(OperandSize::Word), None)), operand2: Some(Literal16(8302)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 134, 100, 46, 18, 104, 110, 32], OperandSize::Qword)
}

#[test]
fn test_47() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDX)), operand2: Some(Literal32(177435610)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 194, 218, 115, 147, 10], OperandSize::Word)
}

#[test]
fn test_48() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1253527632)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 4, 80, 80, 183, 74], OperandSize::Word)
}

#[test]
fn test_49() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDX)), operand2: Some(Literal32(1887021693)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 194, 125, 170, 121, 112], OperandSize::Dword)
}

#[test]
fn test_50() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(615846689)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 1, 33, 19, 181, 36], OperandSize::Dword)
}

#[test]
fn test_51() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDI)), operand2: Some(Literal32(2130718360)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 199, 152, 46, 0, 127], OperandSize::Qword)
}

#[test]
fn test_52() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal32(949682643)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 4, 143, 211, 1, 155, 56], OperandSize::Qword)
}

#[test]
fn test_53() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RSP)), operand2: Some(Literal32(1181472632)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 196, 120, 215, 107, 70], OperandSize::Qword)
}

#[test]
fn test_54() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Literal32(900964214)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 7, 118, 159, 179, 53], OperandSize::Qword)
}

