use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn test_1() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 210], OperandSize::Word)
}

#[test]
fn test_2() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(BX, 27186, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 151, 50, 106], OperandSize::Word)
}

#[test]
fn test_3() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 211], OperandSize::Dword)
}

#[test]
fn test_4() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 31], OperandSize::Dword)
}

#[test]
fn test_5() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 218], OperandSize::Qword)
}

#[test]
fn test_6() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(RBX, 698345991, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 139, 7, 234, 159, 41], OperandSize::Qword)
}

#[test]
fn test_7() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 211], OperandSize::Qword)
}

#[test]
fn test_8() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(RDX, 593931389, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 146, 125, 172, 102, 35], OperandSize::Qword)
}

#[test]
fn test_9() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 206], OperandSize::Word)
}

#[test]
fn test_10() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(DI, 1422, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 173, 142, 5], OperandSize::Word)
}

#[test]
fn test_11() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 247], OperandSize::Dword)
}

#[test]
fn test_12() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 50], OperandSize::Dword)
}

#[test]
fn test_13() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 202], OperandSize::Qword)
}

#[test]
fn test_14() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 2010739104, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 188, 90, 160, 113, 217, 119], OperandSize::Qword)
}

#[test]
fn test_15() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 205], OperandSize::Word)
}

#[test]
fn test_16() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 24685, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 162, 109, 96], OperandSize::Word)
}

#[test]
fn test_17() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 217], OperandSize::Dword)
}

#[test]
fn test_18() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1031892638, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 36, 141, 158, 110, 129, 61], OperandSize::Dword)
}

#[test]
fn test_19() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 253], OperandSize::Qword)
}

#[test]
fn test_20() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RDI, Two, 267349718, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 52, 125, 214, 110, 239, 15], OperandSize::Qword)
}

#[test]
fn test_21() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 133, 223], OperandSize::Qword)
}

#[test]
fn test_22() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 133, 17], OperandSize::Qword)
}

#[test]
fn test_23() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 110], OperandSize::Word)
}

#[test]
fn test_24() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 73], OperandSize::Dword)
}

#[test]
fn test_25() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 84], OperandSize::Qword)
}

#[test]
fn test_26() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(27474)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 82, 107], OperandSize::Word)
}

#[test]
fn test_27() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(25965)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 109, 101], OperandSize::Dword)
}

#[test]
fn test_28() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(22979)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 195, 89], OperandSize::Qword)
}

#[test]
fn test_29() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1251946473)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 233, 47, 159, 74], OperandSize::Word)
}

#[test]
fn test_30() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1564000769)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 1, 194, 56, 93], OperandSize::Dword)
}

#[test]
fn test_31() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1641424487)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 103, 38, 214, 97], OperandSize::Qword)
}

#[test]
fn test_32() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RAX)), operand2: Some(Literal32(235828684)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 169, 204, 117, 14, 14], OperandSize::Qword)
}

#[test]
fn test_33() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 193, 77], OperandSize::Word)
}

#[test]
fn test_34() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(DI, 6388, Some(OperandSize::Byte), None)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 133, 244, 24, 27], OperandSize::Word)
}

#[test]
fn test_35() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 194, 24], OperandSize::Dword)
}

#[test]
fn test_36() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 2048960465, Some(OperandSize::Byte), None)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 132, 215, 209, 167, 32, 122, 83], OperandSize::Dword)
}

#[test]
fn test_37() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 193, 117], OperandSize::Qword)
}

#[test]
fn test_38() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 4, 200, 94], OperandSize::Qword)
}

#[test]
fn test_39() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 194, 91], OperandSize::Qword)
}

#[test]
fn test_40() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 3, 121], OperandSize::Qword)
}

#[test]
fn test_41() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DI)), operand2: Some(Literal16(15856)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 199, 240, 61], OperandSize::Word)
}

#[test]
fn test_42() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: Some(Literal16(7063)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 4, 151, 27], OperandSize::Word)
}

#[test]
fn test_43() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CX)), operand2: Some(Literal16(8357)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 193, 165, 32], OperandSize::Dword)
}

#[test]
fn test_44() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal16(24652)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 4, 95, 76, 96], OperandSize::Dword)
}

#[test]
fn test_45() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BP)), operand2: Some(Literal16(32638)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 197, 126, 127], OperandSize::Qword)
}

#[test]
fn test_46() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal16(5322)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 4, 222, 202, 20], OperandSize::Qword)
}

#[test]
fn test_47() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBX)), operand2: Some(Literal32(523105477)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 195, 197, 244, 45, 31], OperandSize::Word)
}

#[test]
fn test_48() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 37, Some(OperandSize::Dword), None)), operand2: Some(Literal32(767472528)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 66, 37, 144, 179, 190, 45], OperandSize::Word)
}

#[test]
fn test_49() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDI)), operand2: Some(Literal32(1939275455)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 199, 191, 254, 150, 115], OperandSize::Dword)
}

#[test]
fn test_50() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(525021136)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 3, 208, 47, 75, 31], OperandSize::Dword)
}

#[test]
fn test_51() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(ECX)), operand2: Some(Literal32(859732631)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 193, 151, 122, 62, 51], OperandSize::Qword)
}

#[test]
fn test_52() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RSI, Two, 917852109, Some(OperandSize::Dword), None)), operand2: Some(Literal32(763890101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 4, 117, 205, 79, 181, 54, 181, 9, 136, 45], OperandSize::Qword)
}

#[test]
fn test_53() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RDI)), operand2: Some(Literal32(675677585)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 199, 145, 5, 70, 40], OperandSize::Qword)
}

#[test]
fn test_54() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1576403267, Some(OperandSize::Qword), None)), operand2: Some(Literal32(2091559162)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 132, 74, 67, 1, 246, 93, 250, 168, 170, 124], OperandSize::Qword)
}

