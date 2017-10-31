use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sbb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 217], OperandSize::Word)
}

#[test]
fn sbb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(BP, 244, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 150, 244, 0], OperandSize::Word)
}

#[test]
fn sbb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 202], OperandSize::Dword)
}

#[test]
fn sbb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(EDX, Four, 188278381, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 28, 149, 109, 230, 56, 11], OperandSize::Dword)
}

#[test]
fn sbb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 218], OperandSize::Qword)
}

#[test]
fn sbb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 14], OperandSize::Qword)
}

#[test]
fn sbb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 210], OperandSize::Qword)
}

#[test]
fn sbb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 15], OperandSize::Qword)
}

#[test]
fn sbb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 222], OperandSize::Word)
}

#[test]
fn sbb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(BX, 229, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 167, 229, 0], OperandSize::Word)
}

#[test]
fn sbb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 246], OperandSize::Dword)
}

#[test]
fn sbb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 12, 243], OperandSize::Dword)
}

#[test]
fn sbb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 204], OperandSize::Qword)
}

#[test]
fn sbb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RAX, 1199512848, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 144, 16, 29, 127, 71], OperandSize::Qword)
}

#[test]
fn sbb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 252], OperandSize::Word)
}

#[test]
fn sbb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(BX, 136, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 183, 136, 0], OperandSize::Word)
}

#[test]
fn sbb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 219], OperandSize::Dword)
}

#[test]
fn sbb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 1296126836, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 188, 248, 116, 83, 65, 77], OperandSize::Dword)
}

#[test]
fn sbb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 220], OperandSize::Qword)
}

#[test]
fn sbb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1166167201, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 28, 189, 161, 76, 130, 69], OperandSize::Qword)
}

#[test]
fn sbb_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 227], OperandSize::Qword)
}

#[test]
fn sbb_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1309641070, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 180, 91, 110, 137, 15, 78], OperandSize::Qword)
}

#[test]
fn sbb_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 210], OperandSize::Word)
}

#[test]
fn sbb_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(DI, 30338, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 141, 130, 118], OperandSize::Word)
}

#[test]
fn sbb_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 202], OperandSize::Dword)
}

#[test]
fn sbb_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 402705228, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 20, 221, 76, 203, 0, 24], OperandSize::Dword)
}

#[test]
fn sbb_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 217], OperandSize::Qword)
}

#[test]
fn sbb_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 20, 201], OperandSize::Qword)
}

#[test]
fn sbb_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 210], OperandSize::Qword)
}

#[test]
fn sbb_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1230044236, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 28, 77, 76, 252, 80, 73], OperandSize::Qword)
}

#[test]
fn sbb_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 239], OperandSize::Word)
}

#[test]
fn sbb_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(BP, 11520, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 158, 0, 45], OperandSize::Word)
}

#[test]
fn sbb_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 225], OperandSize::Dword)
}

#[test]
fn sbb_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1116147613, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 60, 189, 157, 15, 135, 66], OperandSize::Dword)
}

#[test]
fn sbb_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 228], OperandSize::Qword)
}

#[test]
fn sbb_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 996491961, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 20, 125, 185, 66, 101, 59], OperandSize::Qword)
}

#[test]
fn sbb_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 243], OperandSize::Word)
}

#[test]
fn sbb_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(BX, 163, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 175, 163, 0], OperandSize::Word)
}

#[test]
fn sbb_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 229], OperandSize::Dword)
}

#[test]
fn sbb_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 326254873, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 188, 82, 25, 65, 114, 19], OperandSize::Dword)
}

#[test]
fn sbb_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 251], OperandSize::Qword)
}

#[test]
fn sbb_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 26], OperandSize::Qword)
}

#[test]
fn sbb_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 215], OperandSize::Qword)
}

#[test]
fn sbb_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RSP)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 27, 35], OperandSize::Qword)
}

#[test]
fn sbb_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 35], OperandSize::Word)
}

#[test]
fn sbb_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 13], OperandSize::Dword)
}

#[test]
fn sbb_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 93], OperandSize::Qword)
}

#[test]
fn sbb_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(21148)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 156, 82], OperandSize::Word)
}

#[test]
fn sbb_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(929)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 161, 3], OperandSize::Dword)
}

#[test]
fn sbb_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(24111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 47, 94], OperandSize::Qword)
}

#[test]
fn sbb_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1458996448)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 224, 132, 246, 86], OperandSize::Word)
}

#[test]
fn sbb_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1821378736)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 176, 8, 144, 108], OperandSize::Dword)
}

#[test]
fn sbb_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(296877439)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 127, 253, 177, 17], OperandSize::Qword)
}

#[test]
fn sbb_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1525565683)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 29, 243, 72, 238, 90], OperandSize::Qword)
}

#[test]
fn sbb_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 34], OperandSize::Word)
}

#[test]
fn sbb_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 26527, Some(OperandSize::Byte), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 155, 159, 103, 92], OperandSize::Word)
}

#[test]
fn sbb_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 219, 40], OperandSize::Dword)
}

#[test]
fn sbb_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1337775582, Some(OperandSize::Byte), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 156, 135, 222, 213, 188, 79, 71], OperandSize::Dword)
}

#[test]
fn sbb_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 99], OperandSize::Qword)
}

#[test]
fn sbb_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RAX, Two, 39630104, Some(OperandSize::Byte), None)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 28, 69, 24, 181, 92, 2, 83], OperandSize::Qword)
}

#[test]
fn sbb_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 219, 17], OperandSize::Qword)
}

#[test]
fn sbb_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RBX, 407339011, Some(OperandSize::Byte), None)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 155, 3, 128, 71, 24, 43], OperandSize::Qword)
}

#[test]
fn sbb_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(Literal16(15763)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 219, 147, 61], OperandSize::Word)
}

#[test]
fn sbb_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 25, Some(OperandSize::Word), None)), operand2: Some(Literal16(30350)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 89, 25, 142, 118], OperandSize::Word)
}

#[test]
fn sbb_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Literal16(2576)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 220, 16, 10], OperandSize::Dword)
}

#[test]
fn sbb_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal16(1368)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 28, 66, 88, 5], OperandSize::Dword)
}

#[test]
fn sbb_67() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(Literal16(15922)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 219, 50, 62], OperandSize::Qword)
}

#[test]
fn sbb_68() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Literal16(5911)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 25, 23, 23], OperandSize::Qword)
}

#[test]
fn sbb_69() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESP)), operand2: Some(Literal32(992071199)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 220, 31, 206, 33, 59], OperandSize::Word)
}

#[test]
fn sbb_70() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal32(2021682517)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 27, 85, 109, 128, 120], OperandSize::Word)
}

#[test]
fn sbb_71() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(Literal32(878390433)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 221, 161, 44, 91, 52], OperandSize::Dword)
}

#[test]
fn sbb_72() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(EAX, 1434883098, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1771124664)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 152, 26, 148, 134, 85, 184, 55, 145, 105], OperandSize::Dword)
}

#[test]
fn sbb_73() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Literal32(35857593)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 219, 185, 36, 35, 2], OperandSize::Qword)
}

#[test]
fn sbb_74() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(846562101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 31, 53, 131, 117, 50], OperandSize::Qword)
}

#[test]
fn sbb_75() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RBP)), operand2: Some(Literal32(888546399)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 221, 95, 36, 246, 52], OperandSize::Qword)
}

#[test]
fn sbb_76() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Literal32(974340985)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 27, 121, 67, 19, 58], OperandSize::Qword)
}

#[test]
fn sbb_77() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 223, 113], OperandSize::Word)
}

#[test]
fn sbb_78() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 26538, Some(OperandSize::Word), None)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 154, 170, 103, 108], OperandSize::Word)
}

#[test]
fn sbb_79() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 219, 54], OperandSize::Dword)
}

#[test]
fn sbb_80() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 26, 114], OperandSize::Dword)
}

#[test]
fn sbb_81() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BP)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 221, 69], OperandSize::Qword)
}

#[test]
fn sbb_82() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 30, 36], OperandSize::Qword)
}

#[test]
fn sbb_83() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESI)), operand2: Some(Literal8(124)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 222, 124], OperandSize::Word)
}

#[test]
fn sbb_84() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(BX, 24471, Some(OperandSize::Dword), None)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 159, 151, 95, 64], OperandSize::Word)
}

#[test]
fn sbb_85() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESP)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 220, 40], OperandSize::Dword)
}

#[test]
fn sbb_86() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(EDX, Four, 739517884, Some(OperandSize::Dword), None)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 28, 149, 188, 37, 20, 44, 21], OperandSize::Dword)
}

#[test]
fn sbb_87() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESP)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 220, 77], OperandSize::Qword)
}

#[test]
fn sbb_88() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 31, 2], OperandSize::Qword)
}

#[test]
fn sbb_89() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RSI)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 222, 107], OperandSize::Qword)
}

#[test]
fn sbb_90() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 28, 64, 7], OperandSize::Qword)
}

