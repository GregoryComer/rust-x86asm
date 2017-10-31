use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn or_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 217], OperandSize::Word)
}

#[test]
fn or_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 98, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 90, 98], OperandSize::Word)
}

#[test]
fn or_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 210], OperandSize::Dword)
}

#[test]
fn or_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 28, 135], OperandSize::Dword)
}

#[test]
fn or_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 211], OperandSize::Qword)
}

#[test]
fn or_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 14], OperandSize::Qword)
}

#[test]
fn or_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 219], OperandSize::Qword)
}

#[test]
fn or_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RBX, Two, 2084726152, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 28, 93, 136, 101, 66, 124], OperandSize::Qword)
}

#[test]
fn or_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 218], OperandSize::Word)
}

#[test]
fn or_10() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(BX, 236, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 175, 236, 0], OperandSize::Word)
}

#[test]
fn or_11() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 209], OperandSize::Dword)
}

#[test]
fn or_12() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(ECX, 2093168844, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 145, 204, 56, 195, 124], OperandSize::Dword)
}

#[test]
fn or_13() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 218], OperandSize::Qword)
}

#[test]
fn or_14() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1337118387, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 164, 246, 179, 206, 178, 79], OperandSize::Qword)
}

#[test]
fn or_15() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 225], OperandSize::Word)
}

#[test]
fn or_16() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(DI, 97, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 93, 97], OperandSize::Word)
}

#[test]
fn or_17() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 252], OperandSize::Dword)
}

#[test]
fn or_18() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 38], OperandSize::Dword)
}

#[test]
fn or_19() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 242], OperandSize::Qword)
}

#[test]
fn or_20() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 52, 202], OperandSize::Qword)
}

#[test]
fn or_21() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 225], OperandSize::Qword)
}

#[test]
fn or_22() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RAX, 1751924288, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 152, 64, 62, 108, 104], OperandSize::Qword)
}

#[test]
fn or_23() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 203], OperandSize::Word)
}

#[test]
fn or_24() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Memory(6864, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 30, 208, 26], OperandSize::Word)
}

#[test]
fn or_25() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 217], OperandSize::Dword)
}

#[test]
fn or_26() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 28, 207], OperandSize::Dword)
}

#[test]
fn or_27() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 217], OperandSize::Qword)
}

#[test]
fn or_28() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RDX, 1900066070, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 138, 22, 181, 64, 113], OperandSize::Qword)
}

#[test]
fn or_29() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 209], OperandSize::Qword)
}

#[test]
fn or_30() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 20, 198], OperandSize::Qword)
}

#[test]
fn or_31() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 234], OperandSize::Word)
}

#[test]
fn or_32() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(BP, 6378, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 174, 234, 24], OperandSize::Word)
}

#[test]
fn or_33() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 223], OperandSize::Dword)
}

#[test]
fn or_34() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DI)), operand2: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 57], OperandSize::Dword)
}

#[test]
fn or_35() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 242], OperandSize::Qword)
}

#[test]
fn or_36() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 249519345, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 156, 87, 241, 92, 223, 14], OperandSize::Qword)
}

#[test]
fn or_37() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 225], OperandSize::Word)
}

#[test]
fn or_38() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 32], OperandSize::Word)
}

#[test]
fn or_39() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 206], OperandSize::Dword)
}

#[test]
fn or_40() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 484435346, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 172, 90, 146, 229, 223, 28], OperandSize::Dword)
}

#[test]
fn or_41() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 234], OperandSize::Qword)
}

#[test]
fn or_42() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 52, 202], OperandSize::Qword)
}

#[test]
fn or_43() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 227], OperandSize::Qword)
}

#[test]
fn or_44() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 623435566, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 11, 172, 208, 46, 223, 40, 37], OperandSize::Qword)
}

#[test]
fn or_45() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 90], OperandSize::Word)
}

#[test]
fn or_46() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 0], OperandSize::Dword)
}

#[test]
fn or_47() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 126], OperandSize::Qword)
}

#[test]
fn or_48() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(9602)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 130, 37], OperandSize::Word)
}

#[test]
fn or_49() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(9436)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 220, 36], OperandSize::Dword)
}

#[test]
fn or_50() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(30739)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 19, 120], OperandSize::Qword)
}

#[test]
fn or_51() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(994110113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 161, 234, 64, 59], OperandSize::Word)
}

#[test]
fn or_52() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(442113381)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 101, 29, 90, 26], OperandSize::Dword)
}

#[test]
fn or_53() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1587638257)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 241, 111, 161, 94], OperandSize::Qword)
}

#[test]
fn or_54() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1890863632)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 13, 16, 74, 180, 112], OperandSize::Qword)
}

#[test]
fn or_55() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 203, 4], OperandSize::Word)
}

#[test]
fn or_56() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 28162, Some(OperandSize::Byte), None)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 137, 2, 110, 51], OperandSize::Word)
}

#[test]
fn or_57() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 201, 39], OperandSize::Dword)
}

#[test]
fn or_58() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1794157987, Some(OperandSize::Byte), None)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 12, 245, 163, 173, 240, 106, 109], OperandSize::Dword)
}

#[test]
fn or_59() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 202, 47], OperandSize::Qword)
}

#[test]
fn or_60() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 12, 66, 81], OperandSize::Qword)
}

#[test]
fn or_61() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 202, 79], OperandSize::Qword)
}

#[test]
fn or_62() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RCX, 1782214988, Some(OperandSize::Byte), None)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 137, 76, 113, 58, 106, 18], OperandSize::Qword)
}

#[test]
fn or_63() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Literal16(5194)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 204, 74, 20], OperandSize::Word)
}

#[test]
fn or_64() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Memory(9342, Some(OperandSize::Word), None)), operand2: Some(Literal16(14411)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 14, 126, 36, 75, 56], OperandSize::Word)
}

#[test]
fn or_65() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Literal16(32179)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 202, 179, 125], OperandSize::Dword)
}

#[test]
fn or_66() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Word), None)), operand2: Some(Literal16(25187)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 12, 184, 99, 98], OperandSize::Dword)
}

#[test]
fn or_67() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Literal16(3934)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 202, 94, 15], OperandSize::Qword)
}

#[test]
fn or_68() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: Some(Literal16(5822)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 8, 190, 22], OperandSize::Qword)
}

#[test]
fn or_69() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(Literal32(1042497950)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 204, 158, 65, 35, 62], OperandSize::Word)
}

#[test]
fn or_70() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(2000060761)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 15, 89, 129, 54, 119], OperandSize::Word)
}

#[test]
fn or_71() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Literal32(418736316)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 202, 188, 104, 245, 24], OperandSize::Dword)
}

#[test]
fn or_72() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(47470356)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 10, 20, 87, 212, 2], OperandSize::Dword)
}

#[test]
fn or_73() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBX)), operand2: Some(Literal32(2145649088)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 203, 192, 1, 228, 127], OperandSize::Qword)
}

#[test]
fn or_74() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(142560841)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 11, 73, 78, 127, 8], OperandSize::Qword)
}

#[test]
fn or_75() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RSP)), operand2: Some(Literal32(603791174)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 204, 70, 31, 253, 35], OperandSize::Qword)
}

#[test]
fn or_76() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal32(514009601)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 12, 121, 1, 42, 163, 30], OperandSize::Qword)
}

#[test]
fn or_77() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BP)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 205, 90], OperandSize::Word)
}

#[test]
fn or_78() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 10, 107], OperandSize::Word)
}

#[test]
fn or_79() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 204, 110], OperandSize::Dword)
}

#[test]
fn or_80() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 14, 14], OperandSize::Dword)
}

#[test]
fn or_81() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DI)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 207, 46], OperandSize::Qword)
}

#[test]
fn or_82() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 212197780, Some(OperandSize::Word), None)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 140, 146, 148, 225, 165, 12, 87], OperandSize::Qword)
}

#[test]
fn or_83() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 203, 35], OperandSize::Word)
}

#[test]
fn or_84() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(BP, 145, Some(OperandSize::Dword), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 142, 145, 0, 110], OperandSize::Word)
}

#[test]
fn or_85() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 207, 92], OperandSize::Dword)
}

#[test]
fn or_86() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 9, 7], OperandSize::Dword)
}

#[test]
fn or_87() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 206, 102], OperandSize::Qword)
}

#[test]
fn or_88() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 12, 81, 82], OperandSize::Qword)
}

#[test]
fn or_89() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RDX)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 202, 31], OperandSize::Qword)
}

#[test]
fn or_90() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 12, 73, 40], OperandSize::Qword)
}

