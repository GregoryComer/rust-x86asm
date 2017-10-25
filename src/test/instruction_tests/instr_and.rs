use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn and_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 209], OperandSize::Word)
}

#[test]
fn and_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 9], OperandSize::Word)
}

#[test]
fn and_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 217], OperandSize::Dword)
}

#[test]
fn and_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 9], OperandSize::Dword)
}

#[test]
fn and_5() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 219], OperandSize::Qword)
}

#[test]
fn and_6() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RAX, 491489514, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 152, 234, 136, 75, 29], OperandSize::Qword)
}

#[test]
fn and_7() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 209], OperandSize::Qword)
}

#[test]
fn and_8() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 194861781, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 148, 158, 213, 90, 157, 11], OperandSize::Qword)
}

#[test]
fn and_9() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 245], OperandSize::Word)
}

#[test]
fn and_10() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 31], OperandSize::Word)
}

#[test]
fn and_11() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 220], OperandSize::Dword)
}

#[test]
fn and_12() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 45456975, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 140, 222, 79, 158, 181, 2], OperandSize::Dword)
}

#[test]
fn and_13() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 226], OperandSize::Qword)
}

#[test]
fn and_14() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RAX, 914455538, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 160, 242, 123, 129, 54], OperandSize::Qword)
}

#[test]
fn and_15() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 243], OperandSize::Word)
}

#[test]
fn and_16() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 48], OperandSize::Word)
}

#[test]
fn and_17() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 253], OperandSize::Dword)
}

#[test]
fn and_18() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1287886920, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 156, 251, 72, 152, 195, 76], OperandSize::Dword)
}

#[test]
fn and_19() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 238], OperandSize::Qword)
}

#[test]
fn and_20() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RSI, 1961335065, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 174, 25, 153, 231, 116], OperandSize::Qword)
}

#[test]
fn and_21() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 250], OperandSize::Qword)
}

#[test]
fn and_22() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RCX, 572959668, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 153, 180, 171, 38, 34], OperandSize::Qword)
}

#[test]
fn and_23() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 211], OperandSize::Word)
}

#[test]
fn and_24() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 28], OperandSize::Word)
}

#[test]
fn and_25() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 217], OperandSize::Dword)
}

#[test]
fn and_26() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(IndirectDisplaced(EAX, 152811813, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 152, 37, 185, 27, 9], OperandSize::Dword)
}

#[test]
fn and_27() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 217], OperandSize::Qword)
}

#[test]
fn and_28() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 450340159, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 148, 112, 63, 165, 215, 26], OperandSize::Qword)
}

#[test]
fn and_29() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 209], OperandSize::Qword)
}

#[test]
fn and_30() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(RDX, 1154955424, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 146, 160, 56, 215, 68], OperandSize::Qword)
}

#[test]
fn and_31() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 249], OperandSize::Word)
}

#[test]
fn and_32() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(BX, 26486, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 183, 118, 103], OperandSize::Word)
}

#[test]
fn and_33() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 219], OperandSize::Dword)
}

#[test]
fn and_34() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(EDI, 1322025675, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 143, 203, 130, 204, 78], OperandSize::Dword)
}

#[test]
fn and_35() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 205], OperandSize::Qword)
}

#[test]
fn and_36() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(RBX, 482247633, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 163, 209, 131, 190, 28], OperandSize::Qword)
}

#[test]
fn and_37() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 231], OperandSize::Word)
}

#[test]
fn and_38() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 112, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 97, 112], OperandSize::Word)
}

#[test]
fn and_39() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 238], OperandSize::Dword)
}

#[test]
fn and_40() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1692002594, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 36, 205, 34, 233, 217, 100], OperandSize::Dword)
}

#[test]
fn and_41() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 204], OperandSize::Qword)
}

#[test]
fn and_42() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(RCX, 863348349, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 137, 125, 166, 117, 51], OperandSize::Qword)
}

#[test]
fn and_43() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RBX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 203], OperandSize::Qword)
}

#[test]
fn and_44() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 918484407, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 35, 20, 221, 183, 245, 190, 54], OperandSize::Qword)
}

#[test]
fn and_45() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 2], OperandSize::Word)
}

#[test]
fn and_46() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 87], OperandSize::Dword)
}

#[test]
fn and_47() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 32], OperandSize::Qword)
}

#[test]
fn and_48() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(19369)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 169, 75], OperandSize::Word)
}

#[test]
fn and_49() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(15429)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 69, 60], OperandSize::Dword)
}

#[test]
fn and_50() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(6554)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 154, 25], OperandSize::Qword)
}

#[test]
fn and_51() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(143240391)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 199, 172, 137, 8], OperandSize::Word)
}

#[test]
fn and_52() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(140142151)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 71, 102, 90, 8], OperandSize::Dword)
}

#[test]
fn and_53() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1990059709)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 189, 230, 157, 118], OperandSize::Qword)
}

#[test]
fn and_54() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1939105678)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 37, 142, 103, 148, 115], OperandSize::Qword)
}

#[test]
fn and_55() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 227, 50], OperandSize::Word)
}

#[test]
fn and_56() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 34, 103], OperandSize::Word)
}

#[test]
fn and_57() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 225, 111], OperandSize::Dword)
}

#[test]
fn and_58() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(EDI, 315960782, Some(OperandSize::Byte), None)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 167, 206, 45, 213, 18, 94], OperandSize::Dword)
}

#[test]
fn and_59() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 227, 68], OperandSize::Qword)
}

#[test]
fn and_60() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 38, 62], OperandSize::Qword)
}

#[test]
fn and_61() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 226, 38], OperandSize::Qword)
}

#[test]
fn and_62() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 36, 200, 61], OperandSize::Qword)
}

#[test]
fn and_63() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(Literal16(904)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 230, 136, 3], OperandSize::Word)
}

#[test]
fn and_64() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(DI, 32355, Some(OperandSize::Word), None)), operand2: Some(Literal16(17222)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 165, 99, 126, 70, 67], OperandSize::Word)
}

#[test]
fn and_65() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Literal16(8923)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 226, 219, 34], OperandSize::Dword)
}

#[test]
fn and_66() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Literal16(5030)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 32, 166, 19], OperandSize::Dword)
}

#[test]
fn and_67() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Literal16(31052)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 227, 76, 121], OperandSize::Qword)
}

#[test]
fn and_68() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 886626110, Some(OperandSize::Word), None)), operand2: Some(Literal16(32082)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 36, 205, 62, 215, 216, 52, 82, 125], OperandSize::Qword)
}

#[test]
fn and_69() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Literal32(1822191642)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 227, 26, 112, 156, 108], OperandSize::Word)
}

#[test]
fn and_70() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1579808432)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 32, 176, 246, 41, 94], OperandSize::Word)
}

#[test]
fn and_71() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Literal32(1180931756)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 226, 172, 150, 99, 70], OperandSize::Dword)
}

#[test]
fn and_72() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1270148569)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 33, 217, 237, 180, 75], OperandSize::Dword)
}

#[test]
fn and_73() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Literal32(1191302341)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 226, 197, 212, 1, 71], OperandSize::Qword)
}

#[test]
fn and_74() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(RSI, Two, 416282220, Some(OperandSize::Dword), None)), operand2: Some(Literal32(382478997)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 36, 117, 108, 246, 207, 24, 149, 42, 204, 22], OperandSize::Qword)
}

#[test]
fn and_75() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RBX)), operand2: Some(Literal32(302632514)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 227, 66, 206, 9, 18], OperandSize::Qword)
}

#[test]
fn and_76() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1305032787)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 36, 222, 83, 56, 201, 77], OperandSize::Qword)
}

#[test]
fn and_77() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BP)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 229, 47], OperandSize::Word)
}

#[test]
fn and_78() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 249, Some(OperandSize::Word), None)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 163, 249, 0, 86], OperandSize::Word)
}

#[test]
fn and_79() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BP)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 229, 58], OperandSize::Dword)
}

#[test]
fn and_80() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(EAX, 1574103966, Some(OperandSize::Word), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 160, 158, 235, 210, 93, 66], OperandSize::Dword)
}

#[test]
fn and_81() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 227, 127], OperandSize::Qword)
}

#[test]
fn and_82() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 188802461, Some(OperandSize::Word), None)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 164, 184, 157, 229, 64, 11, 83], OperandSize::Qword)
}

#[test]
fn and_83() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESP)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 228, 91], OperandSize::Word)
}

#[test]
fn and_84() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 35, 14], OperandSize::Word)
}

#[test]
fn and_85() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 226, 38], OperandSize::Dword)
}

#[test]
fn and_86() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 34, 73], OperandSize::Dword)
}

#[test]
fn and_87() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 227, 86], OperandSize::Qword)
}

#[test]
fn and_88() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 36, 128, 68], OperandSize::Qword)
}

#[test]
fn and_89() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RDI)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 231, 98], OperandSize::Qword)
}

#[test]
fn and_90() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RBX, 1598502135, Some(OperandSize::Qword), None)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 163, 247, 52, 71, 95, 52], OperandSize::Qword)
}

