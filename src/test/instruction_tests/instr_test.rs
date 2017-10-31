use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn test_1() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 201], OperandSize::Word)
}

#[test]
fn test_2() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 24], OperandSize::Word)
}

#[test]
fn test_3() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 210], OperandSize::Dword)
}

#[test]
fn test_4() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 20, 154], OperandSize::Dword)
}

#[test]
fn test_5() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 203], OperandSize::Qword)
}

#[test]
fn test_6() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 10], OperandSize::Qword)
}

#[test]
fn test_7() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 217], OperandSize::Qword)
}

#[test]
fn test_8() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[132, 14], OperandSize::Qword)
}

#[test]
fn test_9() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 238], OperandSize::Word)
}

#[test]
fn test_10() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 23], OperandSize::Word)
}

#[test]
fn test_11() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 225], OperandSize::Dword)
}

#[test]
fn test_12() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 50], OperandSize::Dword)
}

#[test]
fn test_13() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 229], OperandSize::Qword)
}

#[test]
fn test_14() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RDX, Four, 150705672, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 28, 149, 8, 150, 251, 8], OperandSize::Qword)
}

#[test]
fn test_15() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 205], OperandSize::Word)
}

#[test]
fn test_16() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(BP, 7096, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 133, 158, 184, 27], OperandSize::Word)
}

#[test]
fn test_17() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 209], OperandSize::Dword)
}

#[test]
fn test_18() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1749019125, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 44, 77, 245, 233, 63, 104], OperandSize::Dword)
}

#[test]
fn test_19() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 221], OperandSize::Qword)
}

#[test]
fn test_20() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(RDX, 1394516053, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[133, 154, 85, 160, 30, 83], OperandSize::Qword)
}

#[test]
fn test_21() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 133, 241], OperandSize::Qword)
}

#[test]
fn test_22() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 133, 50], OperandSize::Qword)
}

#[test]
fn test_23() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 97], OperandSize::Word)
}

#[test]
fn test_24() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 97], OperandSize::Dword)
}

#[test]
fn test_25() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AL)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[168, 9], OperandSize::Qword)
}

#[test]
fn test_26() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(17645)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 237, 68], OperandSize::Word)
}

#[test]
fn test_27() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(15877)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 5, 62], OperandSize::Dword)
}

#[test]
fn test_28() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(AX)), operand2: Some(Literal16(16408)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 24, 64], OperandSize::Qword)
}

#[test]
fn test_29() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(766804824)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 169, 88, 131, 180, 45], OperandSize::Word)
}

#[test]
fn test_30() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1665438321)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 113, 146, 68, 99], OperandSize::Dword)
}

#[test]
fn test_31() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1454013714)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[169, 18, 125, 170, 86], OperandSize::Qword)
}

#[test]
fn test_32() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1079001931)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 169, 75, 67, 80, 64], OperandSize::Qword)
}

#[test]
fn test_33() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 193, 121], OperandSize::Word)
}

#[test]
fn test_34() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 1, 3], OperandSize::Word)
}

#[test]
fn test_35() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(DL)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 194, 89], OperandSize::Dword)
}

#[test]
fn test_36() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 3, 63], OperandSize::Dword)
}

#[test]
fn test_37() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(BL)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 195, 61], OperandSize::Qword)
}

#[test]
fn test_38() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 717437086, Some(OperandSize::Byte), None)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 4, 213, 158, 56, 195, 42, 74], OperandSize::Qword)
}

#[test]
fn test_39() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CL)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 193, 45], OperandSize::Qword)
}

#[test]
fn test_40() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1544121007, Some(OperandSize::Byte), None)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 132, 79, 175, 106, 9, 92, 80], OperandSize::Qword)
}

#[test]
fn test_41() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(SP)), operand2: Some(Literal16(20629)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 196, 149, 80], OperandSize::Word)
}

#[test]
fn test_42() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectDisplaced(BP, 10000, Some(OperandSize::Word), None)), operand2: Some(Literal16(19298)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 134, 16, 39, 98, 75], OperandSize::Word)
}

#[test]
fn test_43() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CX)), operand2: Some(Literal16(21172)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 193, 180, 82], OperandSize::Dword)
}

#[test]
fn test_44() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Literal16(8025)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 0, 89, 31], OperandSize::Dword)
}

#[test]
fn test_45() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(CX)), operand2: Some(Literal16(20048)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 193, 80, 78], OperandSize::Qword)
}

#[test]
fn test_46() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 2111384436, Some(OperandSize::Word), None)), operand2: Some(Literal16(4257)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 132, 88, 116, 43, 217, 125, 161, 16], OperandSize::Qword)
}

#[test]
fn test_47() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EBX)), operand2: Some(Literal32(571153860)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 195, 196, 29, 11, 34], OperandSize::Word)
}

#[test]
fn test_48() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1869945816)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 7, 216, 27, 117, 111], OperandSize::Word)
}

#[test]
fn test_49() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(EDX)), operand2: Some(Literal32(478814092)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 194, 140, 31, 138, 28], OperandSize::Dword)
}

#[test]
fn test_50() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1183347346)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 4, 135, 146, 114, 136, 70], OperandSize::Dword)
}

#[test]
fn test_51() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(ESP)), operand2: Some(Literal32(878860968)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 196, 168, 90, 98, 52], OperandSize::Qword)
}

#[test]
fn test_52() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1869796188, Some(OperandSize::Dword), None)), operand2: Some(Literal32(963363797)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 132, 210, 92, 211, 114, 111, 213, 195, 107, 57], OperandSize::Qword)
}

#[test]
fn test_53() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(Direct(RBP)), operand2: Some(Literal32(1004195377)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 197, 49, 206, 218, 59], OperandSize::Qword)
}

#[test]
fn test_54() {
    run_test(&Instruction { mnemonic: Mnemonic::TEST, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1560538275, Some(OperandSize::Qword), None)), operand2: Some(Literal32(801488707)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 4, 245, 163, 236, 3, 93, 67, 191, 197, 47], OperandSize::Qword)
}

