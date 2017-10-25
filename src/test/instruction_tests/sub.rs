use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 217], OperandSize::Word)
}

fn sub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 29], OperandSize::Word)
}

fn sub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 217], OperandSize::Dword)
}

fn sub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(EAX, 1479472382, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 136, 254, 244, 46, 88], OperandSize::Dword)
}

fn sub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 210], OperandSize::Qword)
}

fn sub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 30], OperandSize::Qword)
}

fn sub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 219], OperandSize::Qword)
}

fn sub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1829550044, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 148, 222, 220, 183, 12, 109], OperandSize::Qword)
}

fn sub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 252], OperandSize::Word)
}

fn sub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 89, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 114, 89], OperandSize::Word)
}

fn sub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 223], OperandSize::Dword)
}

fn sub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1954359038, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 148, 65, 254, 38, 125, 116], OperandSize::Dword)
}

fn sub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 202], OperandSize::Qword)
}

fn sub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 60, 82], OperandSize::Qword)
}

fn sub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 231], OperandSize::Word)
}

fn sub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 562, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 171, 50, 2], OperandSize::Word)
}

fn sub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 219], OperandSize::Dword)
}

fn sub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(EDI, 1734856630, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 167, 182, 207, 103, 103], OperandSize::Dword)
}

fn sub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 238], OperandSize::Qword)
}

fn sub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 24], OperandSize::Qword)
}

fn sub_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 251], OperandSize::Qword)
}

fn sub_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 33], OperandSize::Qword)
}

fn sub_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 219], OperandSize::Word)
}

fn sub_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 26820, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 137, 196, 104], OperandSize::Word)
}

fn sub_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 211], OperandSize::Dword)
}

fn sub_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1039767799, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 140, 198, 247, 152, 249, 61], OperandSize::Dword)
}

fn sub_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 209], OperandSize::Qword)
}

fn sub_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 16], OperandSize::Qword)
}

fn sub_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 219], OperandSize::Qword)
}

fn sub_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1134953299, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 12, 133, 83, 3, 166, 67], OperandSize::Qword)
}

fn sub_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 252], OperandSize::Word)
}

fn sub_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 27, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 81, 27], OperandSize::Word)
}

fn sub_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 223], OperandSize::Dword)
}

fn sub_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(EDI, 968335399, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 175, 39, 160, 183, 57], OperandSize::Dword)
}

fn sub_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 255], OperandSize::Qword)
}

fn sub_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 60, 183], OperandSize::Qword)
}

fn sub_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 205], OperandSize::Word)
}

fn sub_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 26], OperandSize::Word)
}

fn sub_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 245], OperandSize::Dword)
}

fn sub_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 20, 137], OperandSize::Dword)
}

fn sub_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 237], OperandSize::Qword)
}

fn sub_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 831943213, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 12, 205, 45, 114, 150, 49], OperandSize::Qword)
}

fn sub_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 228], OperandSize::Qword)
}

fn sub_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1561319569, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 43, 36, 253, 145, 216, 15, 93], OperandSize::Qword)
}

fn sub_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 119], OperandSize::Word)
}

fn sub_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 54], OperandSize::Dword)
}

fn sub_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 89], OperandSize::Qword)
}

fn sub_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(5496)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 120, 21], OperandSize::Word)
}

fn sub_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(31437)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 205, 122], OperandSize::Dword)
}

fn sub_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(26582)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 214, 103], OperandSize::Qword)
}

fn sub_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1860289428)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 148, 195, 225, 110], OperandSize::Word)
}

fn sub_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(244113670)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 6, 225, 140, 14], OperandSize::Dword)
}

fn sub_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1481137153)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 1, 92, 72, 88], OperandSize::Qword)
}

fn sub_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RAX)), operand2: Some(Literal32(429491497)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 45, 41, 133, 153, 25], OperandSize::Qword)
}

fn sub_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 235, 6], OperandSize::Word)
}

fn sub_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 9908, Some(OperandSize::Byte), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 170, 180, 38, 71], OperandSize::Word)
}

fn sub_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 234, 125], OperandSize::Dword)
}

fn sub_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 2032220497, Some(OperandSize::Byte), None)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 172, 90, 81, 57, 33, 121, 5], OperandSize::Dword)
}

fn sub_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 235, 94], OperandSize::Qword)
}

fn sub_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 46, 17], OperandSize::Qword)
}

fn sub_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 235, 120], OperandSize::Qword)
}

fn sub_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RCX, Two, 911233299, Some(OperandSize::Byte), None)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 44, 77, 19, 81, 80, 54, 53], OperandSize::Qword)
}

fn sub_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CX)), operand2: Some(Literal16(5414)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 233, 38, 21], OperandSize::Word)
}

fn sub_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal16(17697)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 43, 33, 69], OperandSize::Word)
}

fn sub_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SI)), operand2: Some(Literal16(17930)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 238, 10, 70], OperandSize::Dword)
}

fn sub_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: Some(Literal16(15786)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 41, 170, 61], OperandSize::Dword)
}

fn sub_67() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SP)), operand2: Some(Literal16(8457)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 236, 9, 33], OperandSize::Qword)
}

fn sub_68() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RSI, 1879463253, Some(OperandSize::Word), None)), operand2: Some(Literal16(26881)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 174, 85, 85, 6, 112, 1, 105], OperandSize::Qword)
}

fn sub_69() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Literal32(732365803)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 238, 235, 3, 167, 43], OperandSize::Word)
}

fn sub_70() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Memory(28769, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1388699956)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 46, 97, 112, 52, 225, 197, 82], OperandSize::Word)
}

fn sub_71() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDX)), operand2: Some(Literal32(1883855963)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 234, 91, 92, 73, 112], OperandSize::Dword)
}

fn sub_72() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(983911116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 47, 204, 74, 165, 58], OperandSize::Dword)
}

fn sub_73() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ECX)), operand2: Some(Literal32(1674976231)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 233, 231, 27, 214, 99], OperandSize::Qword)
}

fn sub_74() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 2110038710, Some(OperandSize::Dword), None)), operand2: Some(Literal32(340405022)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 172, 183, 182, 162, 196, 125, 30, 43, 74, 20], OperandSize::Qword)
}

fn sub_75() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RDI)), operand2: Some(Literal32(770820064)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 239, 224, 199, 241, 45], OperandSize::Qword)
}

fn sub_76() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RBX, Two, 281635665, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1536805915)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 44, 93, 81, 107, 201, 16, 27, 204, 153, 91], OperandSize::Qword)
}

fn sub_77() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SP)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 236, 58], OperandSize::Word)
}

fn sub_78() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 45, 66], OperandSize::Word)
}

fn sub_79() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 235, 0], OperandSize::Dword)
}

fn sub_80() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(EDI, Four, 681791353, Some(OperandSize::Word), None)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 44, 189, 121, 79, 163, 40, 17], OperandSize::Dword)
}

fn sub_81() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 239, 72], OperandSize::Qword)
}

fn sub_82() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 548430391, Some(OperandSize::Word), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 172, 121, 55, 98, 176, 32, 35], OperandSize::Qword)
}

fn sub_83() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ECX)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 233, 99], OperandSize::Word)
}

fn sub_84() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 43, 107], OperandSize::Word)
}

fn sub_85() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 235, 3], OperandSize::Dword)
}

fn sub_86() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(EDX, 443962204, Some(OperandSize::Dword), None)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 170, 92, 83, 118, 26, 88], OperandSize::Dword)
}

fn sub_87() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 238, 25], OperandSize::Qword)
}

fn sub_88() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1810358792, Some(OperandSize::Dword), None)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 172, 208, 8, 226, 231, 107, 37], OperandSize::Qword)
}

fn sub_89() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RBX)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 235, 39], OperandSize::Qword)
}

fn sub_90() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 64777316, Some(OperandSize::Qword), None)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 172, 202, 100, 108, 220, 3, 53], OperandSize::Qword)
}

