use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn or_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 219], OperandSize::Word)
}

#[test]
fn or_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(BX, 126, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 79, 126], OperandSize::Word)
}

#[test]
fn or_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 203], OperandSize::Dword)
}

#[test]
fn or_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 325403143, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 148, 114, 7, 66, 101, 19], OperandSize::Dword)
}

#[test]
fn or_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 203], OperandSize::Qword)
}

#[test]
fn or_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RAX, 359173983, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 144, 95, 143, 104, 21], OperandSize::Qword)
}

#[test]
fn or_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 217], OperandSize::Qword)
}

#[test]
fn or_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 28, 187], OperandSize::Qword)
}

#[test]
fn or_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 246], OperandSize::Word)
}

#[test]
fn or_10() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(SI, 17886, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 180, 222, 69], OperandSize::Word)
}

#[test]
fn or_11() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 246], OperandSize::Dword)
}

#[test]
fn or_12() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 20, 64], OperandSize::Dword)
}

#[test]
fn or_13() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 243], OperandSize::Qword)
}

#[test]
fn or_14() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 50], OperandSize::Qword)
}

#[test]
fn or_15() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 241], OperandSize::Word)
}

#[test]
fn or_16() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 227, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 152, 227, 0], OperandSize::Word)
}

#[test]
fn or_17() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 242], OperandSize::Dword)
}

#[test]
fn or_18() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1862869517, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 188, 215, 13, 34, 9, 111], OperandSize::Dword)
}

#[test]
fn or_19() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 245], OperandSize::Qword)
}

#[test]
fn or_20() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RBX, Two, 16820367, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 20, 93, 143, 168, 0, 1], OperandSize::Qword)
}

#[test]
fn or_21() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 237], OperandSize::Qword)
}

#[test]
fn or_22() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RDX, Two, 117974300, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 20, 85, 28, 37, 8, 7], OperandSize::Qword)
}

#[test]
fn or_23() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 202], OperandSize::Word)
}

#[test]
fn or_24() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 25], OperandSize::Word)
}

#[test]
fn or_25() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 219], OperandSize::Dword)
}

#[test]
fn or_26() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 2095466409, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 12, 189, 169, 71, 230, 124], OperandSize::Dword)
}

#[test]
fn or_27() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 210], OperandSize::Qword)
}

#[test]
fn or_28() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 12, 194], OperandSize::Qword)
}

#[test]
fn or_29() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 201], OperandSize::Qword)
}

#[test]
fn or_30() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 12, 135], OperandSize::Qword)
}

#[test]
fn or_31() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 252], OperandSize::Word)
}

#[test]
fn or_32() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 20124, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 178, 156, 78], OperandSize::Word)
}

#[test]
fn or_33() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 243], OperandSize::Dword)
}

#[test]
fn or_34() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 274535962, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 20, 213, 26, 22, 93, 16], OperandSize::Dword)
}

#[test]
fn or_35() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 204], OperandSize::Qword)
}

#[test]
fn or_36() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 476875493, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 36, 253, 229, 138, 108, 28], OperandSize::Qword)
}

#[test]
fn or_37() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 210], OperandSize::Word)
}

#[test]
fn or_38() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBX)), operand2: Some(Memory(20500, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 30, 20, 80], OperandSize::Word)
}

#[test]
fn or_39() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 226], OperandSize::Dword)
}

#[test]
fn or_40() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(ESI, 1387237981, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 166, 93, 146, 175, 82], OperandSize::Dword)
}

#[test]
fn or_41() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 225], OperandSize::Qword)
}

#[test]
fn or_42() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 20, 87], OperandSize::Qword)
}

#[test]
fn or_43() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 251], OperandSize::Qword)
}

#[test]
fn or_44() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 297361381, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 11, 140, 126, 229, 95, 185, 17], OperandSize::Qword)
}

#[test]
fn or_45() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 101], OperandSize::Word)
}

#[test]
fn or_46() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 121], OperandSize::Dword)
}

#[test]
fn or_47() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 10], OperandSize::Qword)
}

#[test]
fn or_48() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(18749)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 61, 73], OperandSize::Word)
}

#[test]
fn or_49() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(11773)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 253, 45], OperandSize::Dword)
}

#[test]
fn or_50() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(31662)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 174, 123], OperandSize::Qword)
}

#[test]
fn or_51() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(862092881)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 81, 126, 98, 51], OperandSize::Word)
}

#[test]
fn or_52() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(112401051)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 155, 26, 179, 6], OperandSize::Dword)
}

#[test]
fn or_53() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1079846771)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 115, 39, 93, 64], OperandSize::Qword)
}

#[test]
fn or_54() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RAX)), operand2: Some(Literal32(79526092)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 13, 204, 120, 189, 4], OperandSize::Qword)
}

#[test]
fn or_55() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 201, 116], OperandSize::Word)
}

#[test]
fn or_56() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(BP, 31958, Some(OperandSize::Byte), None)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 142, 214, 124, 91], OperandSize::Word)
}

#[test]
fn or_57() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 202, 46], OperandSize::Dword)
}

#[test]
fn or_58() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 12, 200, 67], OperandSize::Dword)
}

#[test]
fn or_59() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 202, 12], OperandSize::Qword)
}

#[test]
fn or_60() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1469692230, Some(OperandSize::Byte), None)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 140, 208, 70, 185, 153, 87, 95], OperandSize::Qword)
}

#[test]
fn or_61() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 203, 0], OperandSize::Qword)
}

#[test]
fn or_62() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1644029813, Some(OperandSize::Byte), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 12, 77, 117, 231, 253, 97, 107], OperandSize::Qword)
}

#[test]
fn or_63() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CX)), operand2: Some(Literal16(17472)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 201, 64, 68], OperandSize::Word)
}

#[test]
fn or_64() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 216, Some(OperandSize::Word), None)), operand2: Some(Literal16(3226)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 139, 216, 0, 154, 12], OperandSize::Word)
}

#[test]
fn or_65() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Literal16(27019)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 203, 139, 105], OperandSize::Dword)
}

#[test]
fn or_66() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal16(16632)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 12, 83, 248, 64], OperandSize::Dword)
}

#[test]
fn or_67() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DI)), operand2: Some(Literal16(23579)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 207, 27, 92], OperandSize::Qword)
}

#[test]
fn or_68() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RAX, 2117885286, Some(OperandSize::Word), None)), operand2: Some(Literal16(3102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 136, 102, 93, 60, 126, 30, 12], OperandSize::Qword)
}

#[test]
fn or_69() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBX)), operand2: Some(Literal32(318827126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 203, 118, 234, 0, 19], OperandSize::Word)
}

#[test]
fn or_70() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 23095, Some(OperandSize::Dword), None)), operand2: Some(Literal32(783781039)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 137, 55, 90, 175, 140, 183, 46], OperandSize::Word)
}

#[test]
fn or_71() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1803608301)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 205, 237, 224, 128, 107], OperandSize::Dword)
}

#[test]
fn or_72() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(ECX, Four, 239516615, Some(OperandSize::Dword), None)), operand2: Some(Literal32(454735470)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 12, 141, 199, 187, 70, 14, 110, 182, 26, 27], OperandSize::Dword)
}

#[test]
fn or_73() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1550376840)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 205, 136, 223, 104, 92], OperandSize::Qword)
}

#[test]
fn or_74() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1492352757)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 15, 245, 126, 243, 88], OperandSize::Qword)
}

#[test]
fn or_75() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RSI)), operand2: Some(Literal32(1436672518)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 206, 6, 226, 161, 85], OperandSize::Qword)
}

#[test]
fn or_76() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal32(903618041)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 12, 216, 249, 29, 220, 53], OperandSize::Qword)
}

#[test]
fn or_77() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 202, 47], OperandSize::Word)
}

#[test]
fn or_78() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 213, Some(OperandSize::Word), None)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 136, 213, 0, 101], OperandSize::Word)
}

#[test]
fn or_79() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 202, 65], OperandSize::Dword)
}

#[test]
fn or_80() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(EDI, 1491537681, Some(OperandSize::Word), None)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 143, 17, 15, 231, 88, 96], OperandSize::Dword)
}

#[test]
fn or_81() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 204, 65], OperandSize::Qword)
}

#[test]
fn or_82() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 12, 135, 7], OperandSize::Qword)
}

#[test]
fn or_83() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 203, 21], OperandSize::Word)
}

#[test]
fn or_84() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Memory(22932, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 14, 148, 89, 1], OperandSize::Word)
}

#[test]
fn or_85() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 207, 113], OperandSize::Dword)
}

#[test]
fn or_86() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 216787832, Some(OperandSize::Dword), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 140, 91, 120, 235, 235, 12, 115], OperandSize::Dword)
}

#[test]
fn or_87() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 203, 109], OperandSize::Qword)
}

#[test]
fn or_88() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 773854221, Some(OperandSize::Dword), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 140, 67, 13, 20, 32, 46, 113], OperandSize::Qword)
}

#[test]
fn or_89() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RSI)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 206, 119], OperandSize::Qword)
}

#[test]
fn or_90() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1499764778, Some(OperandSize::Qword), None)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 12, 157, 42, 152, 100, 89, 93], OperandSize::Qword)
}

