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
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 223, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 145, 223, 0], OperandSize::Word)
}

#[test]
fn or_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 211], OperandSize::Dword)
}

#[test]
fn or_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(ECX, 1135219285, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 137, 85, 18, 170, 67], OperandSize::Dword)
}

#[test]
fn or_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 203], OperandSize::Qword)
}

#[test]
fn or_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RDI, 1296210857, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 143, 169, 155, 66, 77], OperandSize::Qword)
}

#[test]
fn or_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 209], OperandSize::Qword)
}

#[test]
fn or_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 12, 195], OperandSize::Qword)
}

#[test]
fn or_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 254], OperandSize::Word)
}

#[test]
fn or_10() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(BP, 249, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 182, 249, 0], OperandSize::Word)
}

#[test]
fn or_11() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 204], OperandSize::Dword)
}

#[test]
fn or_12() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(EBX, 1185986195, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 163, 147, 182, 176, 70], OperandSize::Dword)
}

#[test]
fn or_13() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 246], OperandSize::Qword)
}

#[test]
fn or_14() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RDI, 1309673836, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 167, 108, 9, 16, 78], OperandSize::Qword)
}

#[test]
fn or_15() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 238], OperandSize::Word)
}

#[test]
fn or_16() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 36], OperandSize::Word)
}

#[test]
fn or_17() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 213], OperandSize::Dword)
}

#[test]
fn or_18() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 566672483, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 148, 112, 99, 188, 198, 33], OperandSize::Dword)
}

#[test]
fn or_19() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 211], OperandSize::Qword)
}

#[test]
fn or_20() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 10], OperandSize::Qword)
}

#[test]
fn or_21() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 234], OperandSize::Qword)
}

#[test]
fn or_22() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RDX, Two, 2143032163, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 12, 85, 99, 19, 188, 127], OperandSize::Qword)
}

#[test]
fn or_23() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 211], OperandSize::Word)
}

#[test]
fn or_24() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 448, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 147, 192, 1], OperandSize::Word)
}

#[test]
fn or_25() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 211], OperandSize::Dword)
}

#[test]
fn or_26() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 604121303, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 20, 141, 215, 40, 2, 36], OperandSize::Dword)
}

#[test]
fn or_27() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 217], OperandSize::Qword)
}

#[test]
fn or_28() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(RDX, 885776626, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 146, 242, 224, 203, 52], OperandSize::Qword)
}

#[test]
fn or_29() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 201], OperandSize::Qword)
}

#[test]
fn or_30() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 22], OperandSize::Qword)
}

#[test]
fn or_31() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 219], OperandSize::Word)
}

#[test]
fn or_32() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 39], OperandSize::Word)
}

#[test]
fn or_33() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 223], OperandSize::Dword)
}

#[test]
fn or_34() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 221864442, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 44, 125, 250, 97, 57, 13], OperandSize::Dword)
}

#[test]
fn or_35() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 243], OperandSize::Qword)
}

#[test]
fn or_36() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1850871150, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 148, 247, 110, 13, 82, 110], OperandSize::Qword)
}

#[test]
fn or_37() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 252], OperandSize::Word)
}

#[test]
fn or_38() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 16], OperandSize::Word)
}

#[test]
fn or_39() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 233], OperandSize::Dword)
}

#[test]
fn or_40() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBX)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 31], OperandSize::Dword)
}

#[test]
fn or_41() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 218], OperandSize::Qword)
}

#[test]
fn or_42() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1407766296, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 36, 205, 24, 207, 232, 83], OperandSize::Qword)
}

#[test]
fn or_43() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RBX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 203], OperandSize::Qword)
}

#[test]
fn or_44() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RCX)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 11, 11], OperandSize::Qword)
}

#[test]
fn or_45() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 70], OperandSize::Word)
}

#[test]
fn or_46() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 54], OperandSize::Dword)
}

#[test]
fn or_47() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 126], OperandSize::Qword)
}

#[test]
fn or_48() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(1632)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 96, 6], OperandSize::Word)
}

#[test]
fn or_49() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(29084)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 156, 113], OperandSize::Dword)
}

#[test]
fn or_50() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(15656)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 40, 61], OperandSize::Qword)
}

#[test]
fn or_51() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(168816860)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 220, 240, 15, 10], OperandSize::Word)
}

#[test]
fn or_52() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1984894959)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 239, 23, 79, 118], OperandSize::Dword)
}

#[test]
fn or_53() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1665051430)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 38, 171, 62, 99], OperandSize::Qword)
}

#[test]
fn or_54() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RAX)), operand2: Some(Literal32(992469316)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 13, 68, 225, 39, 59], OperandSize::Qword)
}

#[test]
fn or_55() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 202, 56], OperandSize::Word)
}

#[test]
fn or_56() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 11581, Some(OperandSize::Byte), None)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 138, 61, 45, 116], OperandSize::Word)
}

#[test]
fn or_57() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 203, 93], OperandSize::Dword)
}

#[test]
fn or_58() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(ESI, 559817336, Some(OperandSize::Byte), None)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 142, 120, 34, 94, 33, 23], OperandSize::Dword)
}

#[test]
fn or_59() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 202, 97], OperandSize::Qword)
}

#[test]
fn or_60() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 9, 80], OperandSize::Qword)
}

#[test]
fn or_61() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 203, 100], OperandSize::Qword)
}

#[test]
fn or_62() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 2091435550, Some(OperandSize::Byte), None)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 140, 139, 30, 198, 168, 124, 84], OperandSize::Qword)
}

#[test]
fn or_63() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Literal16(12344)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 203, 56, 48], OperandSize::Word)
}

#[test]
fn or_64() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(DI, 64, Some(OperandSize::Word), None)), operand2: Some(Literal16(28439)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 77, 64, 23, 111], OperandSize::Word)
}

#[test]
fn or_65() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Literal16(23451)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 202, 155, 91], OperandSize::Dword)
}

#[test]
fn or_66() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(EBX, 1249552139, Some(OperandSize::Word), None)), operand2: Some(Literal16(25414)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 139, 11, 167, 122, 74, 70, 99], OperandSize::Dword)
}

#[test]
fn or_67() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CX)), operand2: Some(Literal16(15876)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 201, 4, 62], OperandSize::Qword)
}

#[test]
fn or_68() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Literal16(18377)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 9, 201, 71], OperandSize::Qword)
}

#[test]
fn or_69() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Literal32(598780441)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 202, 25, 170, 176, 35], OperandSize::Word)
}

#[test]
fn or_70() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1559844177)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 11, 81, 85, 249, 92], OperandSize::Word)
}

#[test]
fn or_71() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1620438834)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 206, 50, 239, 149, 96], OperandSize::Dword)
}

#[test]
fn or_72() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(EDX, 2000763339, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1054549060)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 138, 203, 57, 65, 119, 68, 36, 219, 62], OperandSize::Dword)
}

#[test]
fn or_73() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBX)), operand2: Some(Literal32(1183018221)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 203, 237, 108, 131, 70], OperandSize::Qword)
}

#[test]
fn or_74() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1394717000)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 12, 194, 72, 177, 33, 83], OperandSize::Qword)
}

#[test]
fn or_75() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RCX)), operand2: Some(Literal32(1998695235)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 201, 67, 171, 33, 119], OperandSize::Qword)
}

#[test]
fn or_76() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1623862597)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 12, 222, 69, 45, 202, 96], OperandSize::Qword)
}

#[test]
fn or_77() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 203, 5], OperandSize::Word)
}

#[test]
fn or_78() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 13, 66], OperandSize::Word)
}

#[test]
fn or_79() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DX)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 202, 25], OperandSize::Dword)
}

#[test]
fn or_80() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 12, 209, 74], OperandSize::Dword)
}

#[test]
fn or_81() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BP)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 205, 72], OperandSize::Qword)
}

#[test]
fn or_82() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 37304605, Some(OperandSize::Word), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 12, 253, 29, 57, 57, 2, 66], OperandSize::Qword)
}

#[test]
fn or_83() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 202, 69], OperandSize::Word)
}

#[test]
fn or_84() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 12, 63], OperandSize::Word)
}

#[test]
fn or_85() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(124)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 202, 124], OperandSize::Dword)
}

#[test]
fn or_86() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1473195647, Some(OperandSize::Dword), None)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 140, 250, 127, 46, 207, 87, 2], OperandSize::Dword)
}

#[test]
fn or_87() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 202, 119], OperandSize::Qword)
}

#[test]
fn or_88() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 12, 187, 48], OperandSize::Qword)
}

#[test]
fn or_89() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RBX)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 203, 38], OperandSize::Qword)
}

#[test]
fn or_90() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RDX, 1518003491, Some(OperandSize::Qword), None)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 138, 35, 229, 122, 90, 62], OperandSize::Qword)
}

