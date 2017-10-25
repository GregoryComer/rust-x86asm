use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 209], OperandSize::Word)
}

#[test]
fn cmp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 140, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 145, 140, 0], OperandSize::Word)
}

#[test]
fn cmp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 209], OperandSize::Dword)
}

#[test]
fn cmp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 16], OperandSize::Dword)
}

#[test]
fn cmp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 201], OperandSize::Qword)
}

#[test]
fn cmp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(RSI, 1478637502, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 158, 190, 55, 34, 88], OperandSize::Qword)
}

#[test]
fn cmp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 218], OperandSize::Qword)
}

#[test]
fn cmp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1732585140, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 20, 69, 180, 38, 69, 103], OperandSize::Qword)
}

#[test]
fn cmp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 245], OperandSize::Word)
}

#[test]
fn cmp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 212, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 161, 212, 0], OperandSize::Word)
}

#[test]
fn cmp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 237], OperandSize::Dword)
}

#[test]
fn cmp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(ECX, 39643242, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 169, 106, 232, 92, 2], OperandSize::Dword)
}

#[test]
fn cmp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 227], OperandSize::Qword)
}

#[test]
fn cmp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1385838973, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 36, 157, 125, 57, 154, 82], OperandSize::Qword)
}

#[test]
fn cmp_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 239], OperandSize::Word)
}

#[test]
fn cmp_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Memory(9345, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 38, 129, 36], OperandSize::Word)
}

#[test]
fn cmp_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 211], OperandSize::Dword)
}

#[test]
fn cmp_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1241054158, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 148, 177, 206, 251, 248, 73], OperandSize::Dword)
}

#[test]
fn cmp_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 222], OperandSize::Qword)
}

#[test]
fn cmp_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 50], OperandSize::Qword)
}

#[test]
fn cmp_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 246], OperandSize::Qword)
}

#[test]
fn cmp_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 2069949107, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 140, 72, 179, 234, 96, 123], OperandSize::Qword)
}

#[test]
fn cmp_23() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 211], OperandSize::Word)
}

#[test]
fn cmp_24() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 160, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 152, 160, 0], OperandSize::Word)
}

#[test]
fn cmp_25() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 211], OperandSize::Dword)
}

#[test]
fn cmp_26() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 28, 135], OperandSize::Dword)
}

#[test]
fn cmp_27() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 219], OperandSize::Qword)
}

#[test]
fn cmp_28() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 25], OperandSize::Qword)
}

#[test]
fn cmp_29() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 209], OperandSize::Qword)
}

#[test]
fn cmp_30() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RAX, 590273718, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 136, 182, 220, 46, 35], OperandSize::Qword)
}

#[test]
fn cmp_31() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 230], OperandSize::Word)
}

#[test]
fn cmp_32() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(SI, 240, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 188, 240, 0], OperandSize::Word)
}

#[test]
fn cmp_33() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 217], OperandSize::Dword)
}

#[test]
fn cmp_34() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 516424955, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 188, 210, 251, 4, 200, 30], OperandSize::Dword)
}

#[test]
fn cmp_35() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 211], OperandSize::Qword)
}

#[test]
fn cmp_36() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 52, 90], OperandSize::Qword)
}

#[test]
fn cmp_37() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 250], OperandSize::Word)
}

#[test]
fn cmp_38() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 29588, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 146, 148, 115], OperandSize::Word)
}

#[test]
fn cmp_39() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 227], OperandSize::Dword)
}

#[test]
fn cmp_40() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 44, 131], OperandSize::Dword)
}

#[test]
fn cmp_41() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 249], OperandSize::Qword)
}

#[test]
fn cmp_42() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RBX, 648207230, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 179, 126, 219, 162, 38], OperandSize::Qword)
}

#[test]
fn cmp_43() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 233], OperandSize::Qword)
}

#[test]
fn cmp_44() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 491735871, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 59, 188, 147, 63, 75, 79, 29], OperandSize::Qword)
}

#[test]
fn cmp_45() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 22], OperandSize::Word)
}

#[test]
fn cmp_46() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 74], OperandSize::Dword)
}

#[test]
fn cmp_47() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 108], OperandSize::Qword)
}

#[test]
fn cmp_48() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(10821)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 69, 42], OperandSize::Word)
}

#[test]
fn cmp_49() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(7920)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 240, 30], OperandSize::Dword)
}

#[test]
fn cmp_50() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(5775)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 143, 22], OperandSize::Qword)
}

#[test]
fn cmp_51() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1610242190)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 142, 88, 250, 95], OperandSize::Word)
}

#[test]
fn cmp_52() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(900333608)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 40, 0, 170, 53], OperandSize::Dword)
}

#[test]
fn cmp_53() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(559090967)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 23, 13, 83, 33], OperandSize::Qword)
}

#[test]
fn cmp_54() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1974119089)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 61, 177, 170, 170, 117], OperandSize::Qword)
}

#[test]
fn cmp_55() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 251, 67], OperandSize::Word)
}

#[test]
fn cmp_56() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 44, Some(OperandSize::Byte), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 121, 44, 81], OperandSize::Word)
}

#[test]
fn cmp_57() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 250, 120], OperandSize::Dword)
}

#[test]
fn cmp_58() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 62, 95], OperandSize::Dword)
}

#[test]
fn cmp_59() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 251, 53], OperandSize::Qword)
}

#[test]
fn cmp_60() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RSI, Two, 1220662891, Some(OperandSize::Byte), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 60, 117, 107, 214, 193, 72, 24], OperandSize::Qword)
}

#[test]
fn cmp_61() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 249, 69], OperandSize::Qword)
}

#[test]
fn cmp_62() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 62, 68], OperandSize::Qword)
}

#[test]
fn cmp_63() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(Literal16(22020)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 250, 4, 86], OperandSize::Word)
}

#[test]
fn cmp_64() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(SI, 31377, Some(OperandSize::Word), None)), operand2: Some(Literal16(23014)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 188, 145, 122, 230, 89], OperandSize::Word)
}

#[test]
fn cmp_65() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BX)), operand2: Some(Literal16(23899)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 251, 91, 93], OperandSize::Dword)
}

#[test]
fn cmp_66() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1988201641, Some(OperandSize::Word), None)), operand2: Some(Literal16(2443)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 60, 189, 169, 140, 129, 118, 139, 9], OperandSize::Dword)
}

#[test]
fn cmp_67() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(Literal16(1530)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 254, 250, 5], OperandSize::Qword)
}

#[test]
fn cmp_68() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal16(29779)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 60, 86, 83, 116], OperandSize::Qword)
}

#[test]
fn cmp_69() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(Literal32(1751055874)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 252, 2, 254, 94, 104], OperandSize::Word)
}

#[test]
fn cmp_70() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1704001454)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 57, 174, 255, 144, 101], OperandSize::Word)
}

#[test]
fn cmp_71() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(Literal32(21854875)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 254, 155, 122, 77, 1], OperandSize::Dword)
}

#[test]
fn cmp_72() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(EAX, Two, 2001396332, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1382466937)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 60, 69, 108, 226, 74, 119, 121, 197, 102, 82], OperandSize::Dword)
}

#[test]
fn cmp_73() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(Literal32(1104380364)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 250, 204, 129, 211, 65], OperandSize::Qword)
}

#[test]
fn cmp_74() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1900623053, Some(OperandSize::Dword), None)), operand2: Some(Literal32(912171589)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 188, 95, 205, 52, 73, 113, 69, 162, 94, 54], OperandSize::Qword)
}

#[test]
fn cmp_75() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RCX)), operand2: Some(Literal32(845124229)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 249, 133, 146, 95, 50], OperandSize::Qword)
}

#[test]
fn cmp_76() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 690147299, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1396388057)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 188, 187, 227, 207, 34, 41, 217, 48, 59, 83], OperandSize::Qword)
}

#[test]
fn cmp_77() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 254, 43], OperandSize::Word)
}

#[test]
fn cmp_78() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 61, 59], OperandSize::Word)
}

#[test]
fn cmp_79() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BX)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 251, 19], OperandSize::Dword)
}

#[test]
fn cmp_80() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 60, 71, 10], OperandSize::Dword)
}

#[test]
fn cmp_81() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BX)), operand2: Some(Literal8(8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 251, 8], OperandSize::Qword)
}

#[test]
fn cmp_82() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 58, 109], OperandSize::Qword)
}

#[test]
fn cmp_83() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBP)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 253, 90], OperandSize::Word)
}

#[test]
fn cmp_84() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 19585, Some(OperandSize::Dword), None)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 187, 129, 76, 58], OperandSize::Word)
}

#[test]
fn cmp_85() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 252, 59], OperandSize::Dword)
}

#[test]
fn cmp_86() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 724788646, Some(OperandSize::Dword), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 60, 205, 166, 101, 51, 43, 115], OperandSize::Dword)
}

#[test]
fn cmp_87() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 252, 47], OperandSize::Qword)
}

#[test]
fn cmp_88() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 56, 35], OperandSize::Qword)
}

#[test]
fn cmp_89() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RSI)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 254, 109], OperandSize::Qword)
}

#[test]
fn cmp_90() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Qword), None)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 60, 146, 36], OperandSize::Qword)
}

