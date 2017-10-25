use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn and_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 217], OperandSize::Word)
}

fn and_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(SI, 179, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 156, 179, 0], OperandSize::Word)
}

fn and_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 203], OperandSize::Dword)
}

fn and_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1410870558, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 140, 75, 30, 45, 24, 84], OperandSize::Dword)
}

fn and_5() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 202], OperandSize::Qword)
}

fn and_6() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 28, 207], OperandSize::Qword)
}

fn and_7() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 201], OperandSize::Qword)
}

fn and_8() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 18], OperandSize::Qword)
}

fn and_9() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 203], OperandSize::Word)
}

fn and_10() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Memory(11663, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 62, 143, 45], OperandSize::Word)
}

fn and_11() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 223], OperandSize::Dword)
}

fn and_12() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 40], OperandSize::Dword)
}

fn and_13() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 233], OperandSize::Qword)
}

fn and_14() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RBX, 1445216715, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 147, 203, 65, 36, 86], OperandSize::Qword)
}

fn and_15() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 250], OperandSize::Word)
}

fn and_16() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Memory(10976, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 54, 224, 42], OperandSize::Word)
}

fn and_17() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 226], OperandSize::Dword)
}

fn and_18() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 32], OperandSize::Dword)
}

fn and_19() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 237], OperandSize::Qword)
}

fn and_20() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 60, 90], OperandSize::Qword)
}

fn and_21() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 221], OperandSize::Qword)
}

fn and_22() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1677811098, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 28, 149, 154, 93, 1, 100], OperandSize::Qword)
}

fn and_23() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 202], OperandSize::Word)
}

fn and_24() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 28790, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 155, 118, 112], OperandSize::Word)
}

fn and_25() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 203], OperandSize::Dword)
}

fn and_26() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 14], OperandSize::Dword)
}

fn and_27() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 210], OperandSize::Qword)
}

fn and_28() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(RSI, 1231238139, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 150, 251, 51, 99, 73], OperandSize::Qword)
}

fn and_29() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 203], OperandSize::Qword)
}

fn and_30() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 557330775, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 148, 176, 87, 49, 56, 33], OperandSize::Qword)
}

fn and_31() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 229], OperandSize::Word)
}

fn and_32() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 34], OperandSize::Word)
}

fn and_33() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 217], OperandSize::Dword)
}

fn and_34() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 428003356, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 148, 145, 28, 208, 130, 25], OperandSize::Dword)
}

fn and_35() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 253], OperandSize::Qword)
}

fn and_36() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 234659511, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 172, 136, 183, 158, 252, 13], OperandSize::Qword)
}

fn and_37() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 238], OperandSize::Word)
}

fn and_38() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 8], OperandSize::Word)
}

fn and_39() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 204], OperandSize::Dword)
}

fn and_40() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(ECX, 533816324, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 137, 4, 100, 209, 31], OperandSize::Dword)
}

fn and_41() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 243], OperandSize::Qword)
}

fn and_42() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 57], OperandSize::Qword)
}

fn and_43() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 235], OperandSize::Qword)
}

fn and_44() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 35, 46], OperandSize::Qword)
}

fn and_45() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 69], OperandSize::Word)
}

fn and_46() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 82], OperandSize::Dword)
}

fn and_47() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 114], OperandSize::Qword)
}

fn and_48() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(14151)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 71, 55], OperandSize::Word)
}

fn and_49() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(28285)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 125, 110], OperandSize::Dword)
}

fn and_50() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(29828)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 132, 116], OperandSize::Qword)
}

fn and_51() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(827733320)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 72, 53, 86, 49], OperandSize::Word)
}

fn and_52() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(766802768)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 80, 123, 180, 45], OperandSize::Dword)
}

fn and_53() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1006595339)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 11, 109, 255, 59], OperandSize::Qword)
}

fn and_54() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RAX)), operand2: Some(Literal32(5194323)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 37, 83, 66, 79, 0], OperandSize::Qword)
}

fn and_55() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 227, 110], OperandSize::Word)
}

fn and_56() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 17904, Some(OperandSize::Byte), None)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 163, 240, 69, 4], OperandSize::Word)
}

fn and_57() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 227, 92], OperandSize::Dword)
}

fn and_58() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 36, 250, 34], OperandSize::Dword)
}

fn and_59() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 225, 45], OperandSize::Qword)
}

fn and_60() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 864754505, Some(OperandSize::Byte), None)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 164, 251, 73, 27, 139, 51, 20], OperandSize::Qword)
}

fn and_61() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 227, 25], OperandSize::Qword)
}

fn and_62() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 36, 246, 8], OperandSize::Qword)
}

fn and_63() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DI)), operand2: Some(Literal16(13549)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 231, 237, 52], OperandSize::Word)
}

fn and_64() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(BX, 138, Some(OperandSize::Word), None)), operand2: Some(Literal16(5416)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 167, 138, 0, 40, 21], OperandSize::Word)
}

fn and_65() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Literal16(14258)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 226, 178, 55], OperandSize::Dword)
}

fn and_66() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1920871663, Some(OperandSize::Word), None)), operand2: Some(Literal16(13369)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 164, 151, 239, 44, 126, 114, 57, 52], OperandSize::Dword)
}

fn and_67() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Literal16(5769)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 226, 137, 22], OperandSize::Qword)
}

fn and_68() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Literal16(13768)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 33, 200, 53], OperandSize::Qword)
}

fn and_69() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Literal32(1017905639)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 231, 231, 1, 172, 60], OperandSize::Word)
}

fn and_70() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 95, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1341457757)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 97, 95, 93, 5, 245, 79], OperandSize::Word)
}

fn and_71() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ECX)), operand2: Some(Literal32(1560601901)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 225, 45, 229, 4, 93], OperandSize::Dword)
}

fn and_72() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1937180474)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 36, 185, 58, 7, 119, 115], OperandSize::Dword)
}

fn and_73() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Literal32(1481816473)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 231, 153, 185, 82, 88], OperandSize::Qword)
}

fn and_74() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1104482359, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1578405565)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 164, 186, 55, 16, 213, 65, 189, 142, 20, 94], OperandSize::Qword)
}

fn and_75() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RSP)), operand2: Some(Literal32(1569529652)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 228, 52, 31, 141, 93], OperandSize::Qword)
}

fn and_76() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Literal32(416833274)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 33, 250, 94, 216, 24], OperandSize::Qword)
}

fn and_77() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 227, 23], OperandSize::Word)
}

fn and_78() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Memory(31639, Some(OperandSize::Word), None)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 38, 151, 123, 117], OperandSize::Word)
}

fn and_79() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SP)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 228, 90], OperandSize::Dword)
}

fn and_80() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(ECX, 1447183493, Some(OperandSize::Word), None)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 161, 133, 68, 66, 86, 11], OperandSize::Dword)
}

fn and_81() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 226, 11], OperandSize::Qword)
}

fn and_82() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 39, 92], OperandSize::Qword)
}

fn and_83() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 231, 15], OperandSize::Word)
}

fn and_84() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(BP, 134, Some(OperandSize::Dword), None)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 166, 134, 0, 19], OperandSize::Word)
}

fn and_85() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 227, 82], OperandSize::Dword)
}

fn and_86() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 1810586690, Some(OperandSize::Dword), None)), operand2: Some(Literal8(13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 164, 209, 66, 92, 235, 107, 13], OperandSize::Dword)
}

fn and_87() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBP)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 229, 101], OperandSize::Qword)
}

fn and_88() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 32, 116], OperandSize::Qword)
}

fn and_89() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RCX)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 225, 93], OperandSize::Qword)
}

fn and_90() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Qword), None)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 36, 191, 93], OperandSize::Qword)
}

