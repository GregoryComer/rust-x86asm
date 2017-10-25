use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sbb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 217], OperandSize::Word)
}

fn sbb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 25684, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 145, 84, 100], OperandSize::Word)
}

fn sbb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 217], OperandSize::Dword)
}

fn sbb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1462687877, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 156, 66, 133, 216, 46, 87], OperandSize::Dword)
}

fn sbb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 210], OperandSize::Qword)
}

fn sbb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 15], OperandSize::Qword)
}

fn sbb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 203], OperandSize::Qword)
}

fn sbb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 2127943264, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 148, 209, 96, 214, 213, 126], OperandSize::Qword)
}

fn sbb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 215], OperandSize::Word)
}

fn sbb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 27], OperandSize::Word)
}

fn sbb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 212], OperandSize::Dword)
}

fn sbb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(EDX, Two, 145225755, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 28, 85, 27, 248, 167, 8], OperandSize::Dword)
}

fn sbb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 250], OperandSize::Qword)
}

fn sbb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RBX, Four, 829520445, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 20, 157, 61, 122, 113, 49], OperandSize::Qword)
}

fn sbb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 207], OperandSize::Word)
}

fn sbb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(BP, 30503, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 182, 39, 119], OperandSize::Word)
}

fn sbb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 220], OperandSize::Dword)
}

fn sbb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 31], OperandSize::Dword)
}

fn sbb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 203], OperandSize::Qword)
}

fn sbb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1308864451, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 20, 149, 195, 175, 3, 78], OperandSize::Qword)
}

fn sbb_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 251], OperandSize::Qword)
}

fn sbb_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 18], OperandSize::Qword)
}

fn sbb_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 202], OperandSize::Word)
}

fn sbb_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 12], OperandSize::Word)
}

fn sbb_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 202], OperandSize::Dword)
}

fn sbb_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(EBX, 1679652706, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 139, 98, 119, 29, 100], OperandSize::Dword)
}

fn sbb_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 203], OperandSize::Qword)
}

fn sbb_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 523539469, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 148, 211, 13, 148, 52, 31], OperandSize::Qword)
}

fn sbb_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 203], OperandSize::Qword)
}

fn sbb_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RDX, 222588000, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 138, 96, 108, 68, 13], OperandSize::Qword)
}

fn sbb_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 217], OperandSize::Word)
}

fn sbb_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(BX, 18877, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 183, 189, 73], OperandSize::Word)
}

fn sbb_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 250], OperandSize::Dword)
}

fn sbb_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 9700959, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 12, 253, 95, 6, 148, 0], OperandSize::Dword)
}

fn sbb_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 206], OperandSize::Qword)
}

fn sbb_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(RDI, 904065724, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 183, 188, 242, 226, 53], OperandSize::Qword)
}

fn sbb_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 231], OperandSize::Word)
}

fn sbb_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 35], OperandSize::Word)
}

fn sbb_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 254], OperandSize::Dword)
}

fn sbb_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EAX, 1258602397, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 168, 157, 191, 4, 75], OperandSize::Dword)
}

fn sbb_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 255], OperandSize::Qword)
}

fn sbb_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 135770228, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 28, 253, 116, 176, 23, 8], OperandSize::Qword)
}

fn sbb_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 230], OperandSize::Qword)
}

fn sbb_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 27, 47], OperandSize::Qword)
}

fn sbb_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 35], OperandSize::Word)
}

fn sbb_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 84], OperandSize::Dword)
}

fn sbb_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 109], OperandSize::Qword)
}

fn sbb_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(23732)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 180, 92], OperandSize::Word)
}

fn sbb_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(31473)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 241, 122], OperandSize::Dword)
}

fn sbb_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(366)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 110, 1], OperandSize::Qword)
}

fn sbb_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1858395163)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 27, 220, 196, 110], OperandSize::Word)
}

fn sbb_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(718707039)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 95, 153, 214, 42], OperandSize::Dword)
}

fn sbb_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1458030393)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 57, 199, 231, 86], OperandSize::Qword)
}

fn sbb_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1156420495)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 29, 143, 147, 237, 68], OperandSize::Qword)
}

fn sbb_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 219, 36], OperandSize::Word)
}

fn sbb_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 72, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 90, 72, 1], OperandSize::Word)
}

fn sbb_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 219, 87], OperandSize::Dword)
}

fn sbb_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 465778490, Some(OperandSize::Byte), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 28, 205, 58, 55, 195, 27, 35], OperandSize::Dword)
}

fn sbb_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 88], OperandSize::Qword)
}

fn sbb_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 31, 77], OperandSize::Qword)
}

fn sbb_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 106], OperandSize::Qword)
}

fn sbb_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RAX, 1676880585, Some(OperandSize::Byte), None)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 152, 201, 42, 243, 99, 83], OperandSize::Qword)
}

fn sbb_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Literal16(20693)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 220, 213, 80], OperandSize::Word)
}

fn sbb_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(BP, 83, Some(OperandSize::Word), None)), operand2: Some(Literal16(12893)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 94, 83, 93, 50], OperandSize::Word)
}

fn sbb_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(Literal16(24454)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 219, 134, 95], OperandSize::Dword)
}

fn sbb_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Literal16(4769)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 24, 161, 18], OperandSize::Dword)
}

fn sbb_67() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(Literal16(6660)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 222, 4, 26], OperandSize::Qword)
}

fn sbb_68() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1465617947, Some(OperandSize::Word), None)), operand2: Some(Literal16(7196)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 28, 189, 27, 142, 91, 87, 28, 28], OperandSize::Qword)
}

fn sbb_69() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1188000927)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 222, 159, 116, 207, 70], OperandSize::Word)
}

fn sbb_70() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1730913961)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 29, 169, 166, 43, 103], OperandSize::Word)
}

fn sbb_71() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDI)), operand2: Some(Literal32(1222878568)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 223, 104, 165, 227, 72], OperandSize::Dword)
}

fn sbb_72() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1439474782)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 28, 186, 94, 164, 204, 85], OperandSize::Dword)
}

fn sbb_73() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(Literal32(203631182)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 221, 78, 42, 35, 12], OperandSize::Qword)
}

fn sbb_74() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 799354968, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1827128220)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 156, 194, 88, 48, 165, 47, 156, 195, 231, 108], OperandSize::Qword)
}

fn sbb_75() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RSP)), operand2: Some(Literal32(1120074393)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 220, 153, 250, 194, 66], OperandSize::Qword)
}

fn sbb_76() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RDX, 1108522033, Some(OperandSize::Qword), None)), operand2: Some(Literal32(987522944)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 154, 49, 180, 18, 66, 128, 103, 220, 58], OperandSize::Qword)
}

fn sbb_77() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 219, 50], OperandSize::Word)
}

fn sbb_78() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(BP, 230, Some(OperandSize::Word), None)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 158, 230, 0, 31], OperandSize::Word)
}

fn sbb_79() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 223, 63], OperandSize::Dword)
}

fn sbb_80() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(EDX, 841489389, Some(OperandSize::Word), None)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 154, 237, 27, 40, 50, 119], OperandSize::Dword)
}

fn sbb_81() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CX)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 217, 60], OperandSize::Qword)
}

fn sbb_82() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 164830388, Some(OperandSize::Word), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 156, 186, 180, 28, 211, 9, 66], OperandSize::Qword)
}

fn sbb_83() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDI)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 223, 63], OperandSize::Word)
}

fn sbb_84() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 24, 49], OperandSize::Word)
}

fn sbb_85() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 221, 6], OperandSize::Dword)
}

fn sbb_86() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 30, 107], OperandSize::Dword)
}

fn sbb_87() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 219, 55], OperandSize::Qword)
}

fn sbb_88() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RAX, 1844593794, Some(OperandSize::Dword), None)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 152, 130, 68, 242, 109, 0], OperandSize::Qword)
}

fn sbb_89() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RCX)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 217, 108], OperandSize::Qword)
}

fn sbb_90() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 28, 193, 3], OperandSize::Qword)
}

