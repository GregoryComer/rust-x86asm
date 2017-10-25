use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn or_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 217], OperandSize::Word)
}

fn or_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(DI, 197, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 149, 197, 0], OperandSize::Word)
}

fn or_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 209], OperandSize::Dword)
}

fn or_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 15], OperandSize::Dword)
}

fn or_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 201], OperandSize::Qword)
}

fn or_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 20, 136], OperandSize::Qword)
}

fn or_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 203], OperandSize::Qword)
}

fn or_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1849309465, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 20, 125, 25, 57, 58, 110], OperandSize::Qword)
}

fn or_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 214], OperandSize::Word)
}

fn or_10() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 5439, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 177, 63, 21], OperandSize::Word)
}

fn or_11() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 225], OperandSize::Dword)
}

fn or_12() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 2052183152, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 164, 80, 112, 212, 81, 122], OperandSize::Dword)
}

fn or_13() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 203], OperandSize::Qword)
}

fn or_14() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RDX, Four, 298041085, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 52, 149, 253, 190, 195, 17], OperandSize::Qword)
}

fn or_15() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 221], OperandSize::Word)
}

fn or_16() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 10197, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 137, 213, 39], OperandSize::Word)
}

fn or_17() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 213], OperandSize::Dword)
}

fn or_18() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1213238997, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 28, 221, 213, 142, 80, 72], OperandSize::Dword)
}

fn or_19() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 209], OperandSize::Qword)
}

fn or_20() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 36, 95], OperandSize::Qword)
}

fn or_21() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 223], OperandSize::Qword)
}

fn or_22() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 712284737, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 172, 118, 65, 154, 116, 42], OperandSize::Qword)
}

fn or_23() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 202], OperandSize::Word)
}

fn or_24() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 17], OperandSize::Word)
}

fn or_25() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 211], OperandSize::Dword)
}

fn or_26() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 9], OperandSize::Dword)
}

fn or_27() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 209], OperandSize::Qword)
}

fn or_28() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 12, 159], OperandSize::Qword)
}

fn or_29() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[8, 201], OperandSize::Qword)
}

fn or_30() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RAX, 155512202, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[10, 136, 138, 237, 68, 9], OperandSize::Qword)
}

fn or_31() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 214], OperandSize::Word)
}

fn or_32() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(DI, 109, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 77, 109], OperandSize::Word)
}

fn or_33() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 220], OperandSize::Dword)
}

fn or_34() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(EDX, 952864828, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 138, 60, 144, 203, 56], OperandSize::Dword)
}

fn or_35() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 228], OperandSize::Qword)
}

fn or_36() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 31], OperandSize::Qword)
}

fn or_37() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 9, 244], OperandSize::Word)
}

fn or_38() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(SI, 31011, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 11, 164, 35, 121], OperandSize::Word)
}

fn or_39() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 223], OperandSize::Dword)
}

fn or_40() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 12, 184], OperandSize::Dword)
}

fn or_41() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[9, 221], OperandSize::Qword)
}

fn or_42() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 374803471, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[11, 172, 137, 15, 12, 87, 22], OperandSize::Qword)
}

fn or_43() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 9, 228], OperandSize::Qword)
}

fn or_44() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 11, 44, 139], OperandSize::Qword)
}

fn or_45() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 35], OperandSize::Word)
}

fn or_46() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 37], OperandSize::Dword)
}

fn or_47() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AL)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[12, 9], OperandSize::Qword)
}

fn or_48() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(15126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 22, 59], OperandSize::Word)
}

fn or_49() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(15430)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 70, 60], OperandSize::Dword)
}

fn or_50() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(AX)), operand2: Some(Literal16(28647)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 231, 111], OperandSize::Qword)
}

fn or_51() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1674409671)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 13, 199, 118, 205, 99], OperandSize::Word)
}

fn or_52() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1690159004)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 156, 199, 189, 100], OperandSize::Dword)
}

fn or_53() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(731810528)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[13, 224, 138, 158, 43], OperandSize::Qword)
}

fn or_54() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RAX)), operand2: Some(Literal32(975181166)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 13, 110, 21, 32, 58], OperandSize::Qword)
}

fn or_55() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 201, 93], OperandSize::Word)
}

fn or_56() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(DI, 4831, Some(OperandSize::Byte), None)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 141, 223, 18, 18], OperandSize::Word)
}

fn or_57() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 201, 53], OperandSize::Dword)
}

fn or_58() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(EDX, Four, 725151930, Some(OperandSize::Byte), None)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 12, 149, 186, 240, 56, 43, 23], OperandSize::Dword)
}

fn or_59() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(CL)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 201, 121], OperandSize::Qword)
}

fn or_60() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 8, 106], OperandSize::Qword)
}

fn or_61() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(DL)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 202, 76], OperandSize::Qword)
}

fn or_62() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1526934125, Some(OperandSize::Byte), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 140, 182, 109, 42, 3, 91, 115], OperandSize::Qword)
}

fn or_63() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Literal16(9249)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 203, 33, 36], OperandSize::Word)
}

fn or_64() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(DI, 27009, Some(OperandSize::Word), None)), operand2: Some(Literal16(15738)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 141, 129, 105, 122, 61], OperandSize::Word)
}

fn or_65() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Literal16(31521)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 203, 33, 123], OperandSize::Dword)
}

fn or_66() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(EAX, 932416725, Some(OperandSize::Word), None)), operand2: Some(Literal16(17016)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 136, 213, 140, 147, 55, 120, 66], OperandSize::Dword)
}

fn or_67() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SI)), operand2: Some(Literal16(32008)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 206, 8, 125], OperandSize::Qword)
}

fn or_68() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Literal16(24449)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 9, 129, 95], OperandSize::Qword)
}

fn or_69() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EBP)), operand2: Some(Literal32(424971532)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 205, 12, 141, 84, 25], OperandSize::Word)
}

fn or_70() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(840985172)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 15, 84, 106, 32, 50], OperandSize::Word)
}

fn or_71() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESI)), operand2: Some(Literal32(377416085)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 206, 149, 233, 126, 22], OperandSize::Dword)
}

fn or_72() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(ESI, 385031325, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1686852551)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 142, 157, 28, 243, 22, 199, 83, 139, 100], OperandSize::Dword)
}

fn or_73() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(Literal32(1282068707)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 204, 227, 208, 106, 76], OperandSize::Qword)
}

fn or_74() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RDX, Four, 694655284, Some(OperandSize::Dword), None)), operand2: Some(Literal32(79947603)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 12, 149, 52, 153, 103, 41, 83, 231, 195, 4], OperandSize::Qword)
}

fn or_75() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RSP)), operand2: Some(Literal32(424612904)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 204, 40, 20, 79, 25], OperandSize::Qword)
}

fn or_76() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1485060717, Some(OperandSize::Qword), None)), operand2: Some(Literal32(772942929)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 12, 197, 109, 58, 132, 88, 81, 44, 18, 46], OperandSize::Qword)
}

fn or_77() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(SP)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 204, 71], OperandSize::Word)
}

fn or_78() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 22330, Some(OperandSize::Word), None)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 138, 58, 87, 6], OperandSize::Word)
}

fn or_79() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BP)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 205, 18], OperandSize::Dword)
}

fn or_80() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(EBX, Four, 391155165, Some(OperandSize::Word), None)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 12, 157, 221, 141, 80, 23, 120], OperandSize::Dword)
}

fn or_81() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(BX)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 203, 55], OperandSize::Qword)
}

fn or_82() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1307846872, Some(OperandSize::Word), None)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 12, 221, 216, 40, 244, 77, 89], OperandSize::Qword)
}

fn or_83() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 201, 18], OperandSize::Word)
}

fn or_84() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Memory(21908, Some(OperandSize::Dword), None)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 14, 148, 85, 83], OperandSize::Word)
}

fn or_85() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 204, 116], OperandSize::Dword)
}

fn or_86() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(EDI, 1278121526, Some(OperandSize::Dword), None)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 143, 54, 150, 46, 76, 116], OperandSize::Dword)
}

fn or_87() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 207, 22], OperandSize::Qword)
}

fn or_88() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1997695064, Some(OperandSize::Dword), None)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 140, 83, 88, 104, 18, 119, 65], OperandSize::Qword)
}

fn or_89() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(Direct(RBX)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 203, 84], OperandSize::Qword)
}

fn or_90() {
    run_test(&Instruction { mnemonic: Mnemonic::OR, operand1: Some(IndirectDisplaced(RCX, 458105039, Some(OperandSize::Qword), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 137, 207, 32, 78, 27, 105], OperandSize::Qword)
}

