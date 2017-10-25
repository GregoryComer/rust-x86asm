use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 203], OperandSize::Word)
}

fn xor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 69, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 91, 69], OperandSize::Word)
}

fn xor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 219], OperandSize::Dword)
}

fn xor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EDX, 125501756, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 138, 60, 1, 123, 7], OperandSize::Dword)
}

fn xor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 203], OperandSize::Qword)
}

fn xor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 710166425, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 156, 200, 153, 71, 84, 42], OperandSize::Qword)
}

fn xor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 209], OperandSize::Qword)
}

fn xor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RBX, 1777028832, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 155, 224, 78, 235, 105], OperandSize::Qword)
}

fn xor_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 204], OperandSize::Word)
}

fn xor_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 31], OperandSize::Word)
}

fn xor_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 251], OperandSize::Dword)
}

fn xor_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1387776849, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 172, 243, 81, 203, 183, 82], OperandSize::Dword)
}

fn xor_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 204], OperandSize::Qword)
}

fn xor_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RDX, 2065516361, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 186, 73, 71, 29, 123], OperandSize::Qword)
}

fn xor_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 228], OperandSize::Word)
}

fn xor_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 47], OperandSize::Word)
}

fn xor_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 207], OperandSize::Dword)
}

fn xor_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EDX, 1301800124, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 162, 188, 228, 151, 77], OperandSize::Dword)
}

fn xor_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 230], OperandSize::Qword)
}

fn xor_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 40], OperandSize::Qword)
}

fn xor_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 253], OperandSize::Qword)
}

fn xor_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RSI, 1563243301, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 174, 37, 51, 45, 93], OperandSize::Qword)
}

fn xor_23() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 203], OperandSize::Word)
}

fn xor_24() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 31], OperandSize::Word)
}

fn xor_25() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 210], OperandSize::Dword)
}

fn xor_26() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 24], OperandSize::Dword)
}

fn xor_27() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 203], OperandSize::Qword)
}

fn xor_28() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1182141724, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 156, 87, 28, 13, 118, 70], OperandSize::Qword)
}

fn xor_29() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 203], OperandSize::Qword)
}

fn xor_30() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(IndirectDisplaced(RBX, 1592556950, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 155, 150, 125, 236, 94], OperandSize::Qword)
}

fn xor_31() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 250], OperandSize::Word)
}

fn xor_32() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 32694, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 138, 182, 127], OperandSize::Word)
}

fn xor_33() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 218], OperandSize::Dword)
}

fn xor_34() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 52, 206], OperandSize::Dword)
}

fn xor_35() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 249], OperandSize::Qword)
}

fn xor_36() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 28, 65], OperandSize::Qword)
}

fn xor_37() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 250], OperandSize::Word)
}

fn xor_38() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 134, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 152, 134, 0], OperandSize::Word)
}

fn xor_39() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 225], OperandSize::Dword)
}

fn xor_40() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(ESI, 400696620, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 142, 44, 37, 226, 23], OperandSize::Dword)
}

fn xor_41() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 204], OperandSize::Qword)
}

fn xor_42() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 62399553, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 60, 125, 65, 36, 184, 3], OperandSize::Qword)
}

fn xor_43() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 249], OperandSize::Qword)
}

fn xor_44() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RDX, 1838888211, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 51, 138, 19, 53, 155, 109], OperandSize::Qword)
}

fn xor_45() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 33], OperandSize::Word)
}

fn xor_46() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 117], OperandSize::Dword)
}

fn xor_47() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 94], OperandSize::Qword)
}

fn xor_48() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(31302)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 70, 122], OperandSize::Word)
}

fn xor_49() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(8157)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 221, 31], OperandSize::Dword)
}

fn xor_50() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(6332)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 188, 24], OperandSize::Qword)
}

fn xor_51() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(236230222)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 78, 150, 20, 14], OperandSize::Word)
}

fn xor_52() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1813791066)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 90, 65, 28, 108], OperandSize::Dword)
}

fn xor_53() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1188928264)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 8, 155, 221, 70], OperandSize::Qword)
}

fn xor_54() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1061145536)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 53, 192, 203, 63, 63], OperandSize::Qword)
}

fn xor_55() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 243, 34], OperandSize::Word)
}

fn xor_56() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 49, 54], OperandSize::Word)
}

fn xor_57() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 241, 104], OperandSize::Dword)
}

fn xor_58() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(ESI, 1324954768, Some(OperandSize::Byte), None)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 182, 144, 52, 249, 78, 111], OperandSize::Dword)
}

fn xor_59() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 242, 113], OperandSize::Qword)
}

fn xor_60() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 51, 28], OperandSize::Qword)
}

fn xor_61() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 242, 1], OperandSize::Qword)
}

fn xor_62() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 52, 134, 24], OperandSize::Qword)
}

fn xor_63() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BX)), operand2: Some(Literal16(13931)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 243, 107, 54], OperandSize::Word)
}

fn xor_64() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 44, Some(OperandSize::Word), None)), operand2: Some(Literal16(22891)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 112, 44, 107, 89], OperandSize::Word)
}

fn xor_65() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Literal16(13214)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 247, 158, 51], OperandSize::Dword)
}

fn xor_66() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1371095532, Some(OperandSize::Word), None)), operand2: Some(Literal16(29179)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 52, 133, 236, 65, 185, 81, 251, 113], OperandSize::Dword)
}

fn xor_67() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(Literal16(28246)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 245, 86, 110], OperandSize::Qword)
}

fn xor_68() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RAX, Two, 498851152, Some(OperandSize::Word), None)), operand2: Some(Literal16(12956)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 52, 69, 80, 221, 187, 29, 156, 50], OperandSize::Qword)
}

fn xor_69() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1700540608)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 246, 192, 48, 92, 101], OperandSize::Word)
}

fn xor_70() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(DI, 28604, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1325711250)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 181, 188, 111, 146, 191, 4, 79], OperandSize::Word)
}

fn xor_71() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1022058870)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 245, 118, 97, 235, 60], OperandSize::Dword)
}

fn xor_72() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EAX, 1821883867, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1494885677)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 176, 219, 189, 151, 108, 45, 37, 26, 89], OperandSize::Dword)
}

fn xor_73() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBX)), operand2: Some(Literal32(575664326)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 243, 198, 240, 79, 34], OperandSize::Qword)
}

fn xor_74() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1136713381, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1702122239)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 180, 190, 165, 222, 192, 67, 255, 82, 116, 101], OperandSize::Qword)
}

fn xor_75() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDI)), operand2: Some(Literal32(1495544683)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 247, 107, 51, 36, 89], OperandSize::Qword)
}

fn xor_76() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Qword), None)), operand2: Some(Literal32(322070855)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 52, 135, 71, 105, 50, 19], OperandSize::Qword)
}

fn xor_77() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 244, 104], OperandSize::Word)
}

fn xor_78() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 74, Some(OperandSize::Word), None)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 112, 74, 101], OperandSize::Word)
}

fn xor_79() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 247, 56], OperandSize::Dword)
}

fn xor_80() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EBX, 11622221, Some(OperandSize::Word), None)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 179, 77, 87, 177, 0, 89], OperandSize::Dword)
}

fn xor_81() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 247, 98], OperandSize::Qword)
}

fn xor_82() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RDX, 428742054, Some(OperandSize::Word), None)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 178, 166, 21, 142, 25, 45], OperandSize::Qword)
}

fn xor_83() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 242, 53], OperandSize::Word)
}

fn xor_84() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(BP, 6987, Some(OperandSize::Dword), None)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 182, 75, 27, 36], OperandSize::Word)
}

fn xor_85() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 245, 15], OperandSize::Dword)
}

fn xor_86() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 240426675, Some(OperandSize::Dword), None)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 180, 142, 179, 158, 84, 14, 98], OperandSize::Dword)
}

fn xor_87() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 245, 96], OperandSize::Qword)
}

fn xor_88() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 295406827, Some(OperandSize::Dword), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 180, 121, 235, 140, 155, 17, 107], OperandSize::Qword)
}

fn xor_89() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RSI)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 246, 116], OperandSize::Qword)
}

fn xor_90() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RAX, 935295945, Some(OperandSize::Qword), None)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 176, 201, 123, 191, 55, 53], OperandSize::Qword)
}

