use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn or_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 203], OperandSize::Word)
}

#[test]
fn or_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(BP, 5, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 78, 5], OperandSize::Word)
}

#[test]
fn or_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 218], OperandSize::Dword)
}

#[test]
fn or_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 28, 129], OperandSize::Dword)
}

#[test]
fn or_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 219], OperandSize::Qword)
}

#[test]
fn or_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1246712239, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 140, 112, 175, 81, 79, 74], OperandSize::Qword)
}

#[test]
fn or_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 203], OperandSize::Qword)
}

#[test]
fn or_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 12, 122], OperandSize::Qword)
}

#[test]
fn or_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 220], OperandSize::Word)
}

#[test]
fn or_10() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(DI, 202, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 149, 202, 0], OperandSize::Word)
}

#[test]
fn or_11() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 250], OperandSize::Dword)
}

#[test]
fn or_12() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 52, 216], OperandSize::Dword)
}

#[test]
fn or_13() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 212], OperandSize::Qword)
}

#[test]
fn or_14() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RAX, 1650534792, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 168, 136, 41, 97, 98], OperandSize::Qword)
}

#[test]
fn or_15() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 214], OperandSize::Word)
}

#[test]
fn or_16() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 41], OperandSize::Word)
}

#[test]
fn or_17() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 251], OperandSize::Dword)
}

#[test]
fn or_18() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 16], OperandSize::Dword)
}

#[test]
fn or_19() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 201], OperandSize::Qword)
}

#[test]
fn or_20() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 28, 113], OperandSize::Qword)
}

#[test]
fn or_21() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 217], OperandSize::Qword)
}

#[test]
fn or_22() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 654296741, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 140, 203, 165, 198, 255, 38], OperandSize::Qword)
}

#[test]
fn or_23() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 202], OperandSize::Word)
}

#[test]
fn or_24() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 31], OperandSize::Word)
}

#[test]
fn or_25() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 217], OperandSize::Dword)
}

#[test]
fn or_26() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 20, 126], OperandSize::Dword)
}

#[test]
fn or_27() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 218], OperandSize::Qword)
}

#[test]
fn or_28() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RDI, 575500068, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 143, 36, 111, 77, 34], OperandSize::Qword)
}

#[test]
fn or_29() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 209], OperandSize::Qword)
}

#[test]
fn or_30() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 770251390, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 20, 125, 126, 26, 233, 45], OperandSize::Qword)
}

#[test]
fn or_31() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 203], OperandSize::Word)
}

#[test]
fn or_32() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CX)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 15], OperandSize::Word)
}

#[test]
fn or_33() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 211], OperandSize::Dword)
}

#[test]
fn or_34() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CX)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 8], OperandSize::Dword)
}

#[test]
fn or_35() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 206], OperandSize::Qword)
}

#[test]
fn or_36() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(RBX, 132362408, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 147, 168, 176, 227, 7], OperandSize::Qword)
}

#[test]
fn or_37() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 222], OperandSize::Word)
}

#[test]
fn or_38() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(Memory(10469, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 38, 229, 40], OperandSize::Word)
}

#[test]
fn or_39() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 238], OperandSize::Dword)
}

#[test]
fn or_40() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1815605752, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 44, 85, 248, 241, 55, 108], OperandSize::Dword)
}

#[test]
fn or_41() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 249], OperandSize::Qword)
}

#[test]
fn or_42() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 17], OperandSize::Qword)
}

#[test]
fn or_43() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 234], OperandSize::Qword)
}

#[test]
fn or_44() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1748530331, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 11, 28, 189, 155, 116, 56, 104], OperandSize::Qword)
}

#[test]
fn or_45() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 70], OperandSize::Word)
}

#[test]
fn or_46() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 70], OperandSize::Dword)
}

#[test]
fn or_47() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 80], OperandSize::Qword)
}

#[test]
fn or_48() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(17704)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 40, 69], OperandSize::Word)
}

#[test]
fn or_49() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(31342)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 110, 122], OperandSize::Dword)
}

#[test]
fn or_50() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(18025)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 105, 70], OperandSize::Qword)
}

#[test]
fn or_51() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1289648969)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 73, 123, 222, 76], OperandSize::Word)
}

#[test]
fn or_52() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1505733827)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 195, 172, 191, 89], OperandSize::Dword)
}

#[test]
fn or_53() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(762151307)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 139, 129, 109, 45], OperandSize::Qword)
}

#[test]
fn or_54() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1546424844)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 13, 12, 146, 44, 92], OperandSize::Qword)
}

#[test]
fn or_55() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 201, 52], OperandSize::Word)
}

#[test]
fn or_56() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(BX, 24224, Some(OperandSize::Byte), None)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 143, 160, 94, 119], OperandSize::Word)
}

#[test]
fn or_57() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 202, 90], OperandSize::Dword)
}

#[test]
fn or_58() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(EBX, Four, 786253458, Some(OperandSize::Byte), None)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 12, 157, 146, 70, 221, 46, 15], OperandSize::Dword)
}

#[test]
fn or_59() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 201, 0], OperandSize::Qword)
}

#[test]
fn or_60() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RDX, Four, 915058550, Some(OperandSize::Byte), None)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 12, 149, 118, 175, 138, 54, 74], OperandSize::Qword)
}

#[test]
fn or_61() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Literal8(124)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 201, 124], OperandSize::Qword)
}

#[test]
fn or_62() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 8, 121], OperandSize::Qword)
}

#[test]
fn or_63() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BP)), operand2: Some(Literal16(25000)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 205, 168, 97], OperandSize::Word)
}

#[test]
fn or_64() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(BX, 36, Some(OperandSize::Word), None)), operand2: Some(Literal16(19238)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 79, 36, 38, 75], OperandSize::Word)
}

#[test]
fn or_65() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SI)), operand2: Some(Literal16(30249)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 206, 41, 118], OperandSize::Dword)
}

#[test]
fn or_66() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(ESI, Four, 832698642, Some(OperandSize::Word), None)), operand2: Some(Literal16(1490)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 12, 181, 18, 249, 161, 49, 210, 5], OperandSize::Dword)
}

#[test]
fn or_67() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Literal16(16387)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 202, 3, 64], OperandSize::Qword)
}

#[test]
fn or_68() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1738042463, Some(OperandSize::Word), None)), operand2: Some(Literal16(17671)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 12, 205, 95, 108, 152, 103, 7, 69], OperandSize::Qword)
}

#[test]
fn or_69() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Literal32(804535227)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 205, 187, 59, 244, 47], OperandSize::Word)
}

#[test]
fn or_70() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 127, Some(OperandSize::Dword), None)), operand2: Some(Literal32(640143960)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 75, 127, 88, 210, 39, 38], OperandSize::Word)
}

#[test]
fn or_71() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1011433539)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 205, 67, 64, 73, 60], OperandSize::Dword)
}

#[test]
fn or_72() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1389687616, Some(OperandSize::Dword), None)), operand2: Some(Literal32(414062560)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 140, 113, 64, 243, 212, 82, 224, 23, 174, 24], OperandSize::Dword)
}

#[test]
fn or_73() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Literal32(798764586)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 205, 42, 46, 156, 47], OperandSize::Qword)
}

#[test]
fn or_74() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1218878944, Some(OperandSize::Dword), None)), operand2: Some(Literal32(691916227)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 12, 213, 224, 157, 166, 72, 195, 205, 61, 41], OperandSize::Qword)
}

#[test]
fn or_75() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RSI)), operand2: Some(Literal32(1348136650)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 206, 202, 238, 90, 80], OperandSize::Qword)
}

#[test]
fn or_76() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RDX, 182157509, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1217937961)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 138, 197, 128, 219, 10, 41, 66, 152, 72], OperandSize::Qword)
}

#[test]
fn or_77() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DI)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 207, 99], OperandSize::Word)
}

#[test]
fn or_78() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 5, Some(OperandSize::Word), None)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 74, 5, 48], OperandSize::Word)
}

#[test]
fn or_79() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CX)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 201, 123], OperandSize::Dword)
}

#[test]
fn or_80() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1754635646, Some(OperandSize::Word), None)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 140, 67, 126, 157, 149, 104, 99], OperandSize::Dword)
}

#[test]
fn or_81() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 203, 40], OperandSize::Qword)
}

#[test]
fn or_82() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 8, 88], OperandSize::Qword)
}

#[test]
fn or_83() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 207, 101], OperandSize::Word)
}

#[test]
fn or_84() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 0, Some(OperandSize::Dword), None)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 9, 106], OperandSize::Word)
}

#[test]
fn or_85() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 204, 92], OperandSize::Dword)
}

#[test]
fn or_86() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 12, 72, 1], OperandSize::Dword)
}

#[test]
fn or_87() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 202, 76], OperandSize::Qword)
}

#[test]
fn or_88() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 223105124, Some(OperandSize::Dword), None)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 140, 90, 100, 80, 76, 13, 31], OperandSize::Qword)
}

#[test]
fn or_89() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RSP)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 204, 35], OperandSize::Qword)
}

#[test]
fn or_90() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RDI, 2066681768, Some(OperandSize::Qword), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 143, 168, 15, 47, 123, 82], OperandSize::Qword)
}

