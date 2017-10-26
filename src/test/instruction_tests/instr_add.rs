use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn add_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 210], OperandSize::Word)
}

#[test]
fn add_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 204, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 136, 204, 0], OperandSize::Word)
}

#[test]
fn add_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 201], OperandSize::Dword)
}

#[test]
fn add_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 19], OperandSize::Dword)
}

#[test]
fn add_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 209], OperandSize::Qword)
}

#[test]
fn add_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RBX, 2105232598, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 155, 214, 76, 123, 125], OperandSize::Qword)
}

#[test]
fn add_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 202], OperandSize::Qword)
}

#[test]
fn add_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 12, 192], OperandSize::Qword)
}

#[test]
fn add_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 249], OperandSize::Word)
}

#[test]
fn add_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 195, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 179, 195, 0], OperandSize::Word)
}

#[test]
fn add_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 218], OperandSize::Dword)
}

#[test]
fn add_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 180960445, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 164, 203, 189, 60, 201, 10], OperandSize::Dword)
}

#[test]
fn add_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 219], OperandSize::Qword)
}

#[test]
fn add_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 24], OperandSize::Qword)
}

#[test]
fn add_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 207], OperandSize::Word)
}

#[test]
fn add_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Memory(12451, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 62, 163, 48], OperandSize::Word)
}

#[test]
fn add_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 219], OperandSize::Dword)
}

#[test]
fn add_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(EAX, 1592809719, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 168, 247, 88, 240, 94], OperandSize::Dword)
}

#[test]
fn add_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 220], OperandSize::Qword)
}

#[test]
fn add_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1767145296, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 156, 195, 80, 127, 84, 105], OperandSize::Qword)
}

#[test]
fn add_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 245], OperandSize::Qword)
}

#[test]
fn add_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 41], OperandSize::Qword)
}

#[test]
fn add_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 218], OperandSize::Word)
}

#[test]
fn add_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 13], OperandSize::Word)
}

#[test]
fn add_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 210], OperandSize::Dword)
}

#[test]
fn add_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 12, 193], OperandSize::Dword)
}

#[test]
fn add_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 211], OperandSize::Qword)
}

#[test]
fn add_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(IndirectDisplaced(RDI, 826057767, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 159, 39, 164, 60, 49], OperandSize::Qword)
}

#[test]
fn add_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 209], OperandSize::Qword)
}

#[test]
fn add_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 16], OperandSize::Qword)
}

#[test]
fn add_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 205], OperandSize::Word)
}

#[test]
fn add_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(BP, 152, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 182, 152, 0], OperandSize::Word)
}

#[test]
fn add_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 223], OperandSize::Dword)
}

#[test]
fn add_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 32], OperandSize::Dword)
}

#[test]
fn add_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 201], OperandSize::Qword)
}

#[test]
fn add_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 56], OperandSize::Qword)
}

#[test]
fn add_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 237], OperandSize::Word)
}

#[test]
fn add_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 59], OperandSize::Word)
}

#[test]
fn add_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 230], OperandSize::Dword)
}

#[test]
fn add_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 12, 120], OperandSize::Dword)
}

#[test]
fn add_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 237], OperandSize::Qword)
}

#[test]
fn add_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 12, 75], OperandSize::Qword)
}

#[test]
fn add_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 233], OperandSize::Qword)
}

#[test]
fn add_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RSI)), operand2: Some(IndirectDisplaced(RDI, 878099426, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 3, 183, 226, 187, 86, 52], OperandSize::Qword)
}

#[test]
fn add_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 58], OperandSize::Word)
}

#[test]
fn add_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 85], OperandSize::Dword)
}

#[test]
fn add_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 78], OperandSize::Qword)
}

#[test]
fn add_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(11709)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 189, 45], OperandSize::Word)
}

#[test]
fn add_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(25806)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 206, 100], OperandSize::Dword)
}

#[test]
fn add_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(32071)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 71, 125], OperandSize::Qword)
}

#[test]
fn add_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(863209858)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 130, 137, 115, 51], OperandSize::Word)
}

#[test]
fn add_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(846765881)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 57, 159, 120, 50], OperandSize::Dword)
}

#[test]
fn add_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(925220652)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 44, 191, 37, 55], OperandSize::Qword)
}

#[test]
fn add_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1037801366)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 5, 150, 151, 219, 61], OperandSize::Qword)
}

#[test]
fn add_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 193, 33], OperandSize::Word)
}

#[test]
fn add_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 22672, Some(OperandSize::Byte), None)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 131, 144, 88, 29], OperandSize::Word)
}

#[test]
fn add_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 195, 61], OperandSize::Dword)
}

#[test]
fn add_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 4, 194, 10], OperandSize::Dword)
}

#[test]
fn add_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 194, 57], OperandSize::Qword)
}

#[test]
fn add_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 1, 1], OperandSize::Qword)
}

#[test]
fn add_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 193, 0], OperandSize::Qword)
}

#[test]
fn add_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1278651352, Some(OperandSize::Byte), None)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 132, 90, 216, 171, 54, 76, 11], OperandSize::Qword)
}

#[test]
fn add_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BP)), operand2: Some(Literal16(19652)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 197, 196, 76], OperandSize::Word)
}

#[test]
fn add_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal16(18291)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 3, 115, 71], OperandSize::Word)
}

#[test]
fn add_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Literal16(22388)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 194, 116, 87], OperandSize::Dword)
}

#[test]
fn add_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal16(24187)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 4, 89, 123, 94], OperandSize::Dword)
}

#[test]
fn add_67() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BX)), operand2: Some(Literal16(3984)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 195, 144, 15], OperandSize::Qword)
}

#[test]
fn add_68() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal16(17571)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 4, 74, 163, 68], OperandSize::Qword)
}

#[test]
fn add_69() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDI)), operand2: Some(Literal32(622714858)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 199, 234, 223, 29, 37], OperandSize::Word)
}

#[test]
fn add_70() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 15222, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1940313502)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 130, 118, 59, 158, 213, 166, 115], OperandSize::Word)
}

#[test]
fn add_71() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Literal32(866581996)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 197, 236, 253, 166, 51], OperandSize::Dword)
}

#[test]
fn add_72() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1573017467, Some(OperandSize::Dword), None)), operand2: Some(Literal32(814720440)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 132, 127, 123, 87, 194, 93, 184, 165, 143, 48], OperandSize::Dword)
}

#[test]
fn add_73() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESP)), operand2: Some(Literal32(599347978)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 196, 10, 83, 185, 35], OperandSize::Qword)
}

#[test]
fn add_74() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 905723201, Some(OperandSize::Dword), None)), operand2: Some(Literal32(393202327)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 132, 217, 65, 61, 252, 53, 151, 202, 111, 23], OperandSize::Qword)
}

#[test]
fn add_75() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RSP)), operand2: Some(Literal32(612869915)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 196, 27, 167, 135, 36], OperandSize::Qword)
}

#[test]
fn add_76() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RDI, 1115202072, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1928654964)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 135, 24, 162, 120, 66, 116, 240, 244, 114], OperandSize::Qword)
}

#[test]
fn add_77() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 199, 57], OperandSize::Word)
}

#[test]
fn add_78() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 0, 44], OperandSize::Word)
}

#[test]
fn add_79() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 198, 15], OperandSize::Dword)
}

#[test]
fn add_80() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1924573915, Some(OperandSize::Word), None)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 132, 147, 219, 170, 182, 114, 93], OperandSize::Dword)
}

#[test]
fn add_81() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 196, 19], OperandSize::Qword)
}

#[test]
fn add_82() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 180763479, Some(OperandSize::Word), None)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 132, 201, 87, 59, 198, 10, 74], OperandSize::Qword)
}

#[test]
fn add_83() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESP)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 196, 64], OperandSize::Word)
}

#[test]
fn add_84() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 13612, Some(OperandSize::Dword), None)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 129, 44, 53, 108], OperandSize::Word)
}

#[test]
fn add_85() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 197, 86], OperandSize::Dword)
}

#[test]
fn add_86() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(EDI, 1174537492, Some(OperandSize::Dword), None)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 135, 20, 5, 2, 70, 15], OperandSize::Dword)
}

#[test]
fn add_87() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 197, 46], OperandSize::Qword)
}

#[test]
fn add_88() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(124)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 0, 124], OperandSize::Qword)
}

#[test]
fn add_89() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RBP)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 197, 125], OperandSize::Qword)
}

#[test]
fn add_90() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 4, 199, 44], OperandSize::Qword)
}

