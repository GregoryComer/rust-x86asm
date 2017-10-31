use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 203], OperandSize::Word)
}

#[test]
fn cmp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 51, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 72, 51], OperandSize::Word)
}

#[test]
fn cmp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 217], OperandSize::Dword)
}

#[test]
fn cmp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 28, 72], OperandSize::Dword)
}

#[test]
fn cmp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 202], OperandSize::Qword)
}

#[test]
fn cmp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(RDX, 1483412440, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 146, 216, 19, 107, 88], OperandSize::Qword)
}

#[test]
fn cmp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 217], OperandSize::Qword)
}

#[test]
fn cmp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 25], OperandSize::Qword)
}

#[test]
fn cmp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 213], OperandSize::Word)
}

#[test]
fn cmp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 31], OperandSize::Word)
}

#[test]
fn cmp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 231], OperandSize::Dword)
}

#[test]
fn cmp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 51], OperandSize::Dword)
}

#[test]
fn cmp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 226], OperandSize::Qword)
}

#[test]
fn cmp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 380535582, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 140, 127, 30, 131, 174, 22], OperandSize::Qword)
}

#[test]
fn cmp_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 238], OperandSize::Word)
}

#[test]
fn cmp_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 20], OperandSize::Word)
}

#[test]
fn cmp_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 231], OperandSize::Dword)
}

#[test]
fn cmp_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 25], OperandSize::Dword)
}

#[test]
fn cmp_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 209], OperandSize::Qword)
}

#[test]
fn cmp_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 335201386, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 148, 201, 106, 196, 250, 19], OperandSize::Qword)
}

#[test]
fn cmp_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 234], OperandSize::Qword)
}

#[test]
fn cmp_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 39], OperandSize::Qword)
}

#[test]
fn cmp_23() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 210], OperandSize::Word)
}

#[test]
fn cmp_24() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 16], OperandSize::Word)
}

#[test]
fn cmp_25() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 211], OperandSize::Dword)
}

#[test]
fn cmp_26() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 22], OperandSize::Dword)
}

#[test]
fn cmp_27() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 209], OperandSize::Qword)
}

#[test]
fn cmp_28() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(RDX, 959591118, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 146, 206, 50, 50, 57], OperandSize::Qword)
}

#[test]
fn cmp_29() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 219], OperandSize::Qword)
}

#[test]
fn cmp_30() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RDX, 1960069414, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 138, 38, 73, 212, 116], OperandSize::Qword)
}

#[test]
fn cmp_31() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 251], OperandSize::Word)
}

#[test]
fn cmp_32() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(DI, 227, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 181, 227, 0], OperandSize::Word)
}

#[test]
fn cmp_33() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 241], OperandSize::Dword)
}

#[test]
fn cmp_34() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(EDI, 518438738, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 143, 82, 191, 230, 30], OperandSize::Dword)
}

#[test]
fn cmp_35() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 206], OperandSize::Qword)
}

#[test]
fn cmp_36() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CX)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 9], OperandSize::Qword)
}

#[test]
fn cmp_37() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 239], OperandSize::Word)
}

#[test]
fn cmp_38() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 28281, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 146, 121, 110], OperandSize::Word)
}

#[test]
fn cmp_39() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 218], OperandSize::Dword)
}

#[test]
fn cmp_40() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 68689383, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 36, 205, 231, 29, 24, 4], OperandSize::Dword)
}

#[test]
fn cmp_41() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 251], OperandSize::Qword)
}

#[test]
fn cmp_42() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(RDI, 698213837, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 143, 205, 229, 157, 41], OperandSize::Qword)
}

#[test]
fn cmp_43() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 237], OperandSize::Qword)
}

#[test]
fn cmp_44() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 25575614, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 59, 52, 205, 190, 64, 134, 1], OperandSize::Qword)
}

#[test]
fn cmp_45() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 70], OperandSize::Word)
}

#[test]
fn cmp_46() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 96], OperandSize::Dword)
}

#[test]
fn cmp_47() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 106], OperandSize::Qword)
}

#[test]
fn cmp_48() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(9635)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 163, 37], OperandSize::Word)
}

#[test]
fn cmp_49() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(8795)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 91, 34], OperandSize::Dword)
}

#[test]
fn cmp_50() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(15047)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 199, 58], OperandSize::Qword)
}

#[test]
fn cmp_51() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1419683236)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 164, 165, 158, 84], OperandSize::Word)
}

#[test]
fn cmp_52() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(187378855)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 167, 44, 43, 11], OperandSize::Dword)
}

#[test]
fn cmp_53() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(2074246572)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 172, 125, 162, 123], OperandSize::Qword)
}

#[test]
fn cmp_54() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1103433862)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 61, 134, 16, 197, 65], OperandSize::Qword)
}

#[test]
fn cmp_55() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 249, 109], OperandSize::Word)
}

#[test]
fn cmp_56() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 30180, Some(OperandSize::Byte), None)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 187, 228, 117, 43], OperandSize::Word)
}

#[test]
fn cmp_57() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 251, 51], OperandSize::Dword)
}

#[test]
fn cmp_58() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 63, 22], OperandSize::Dword)
}

#[test]
fn cmp_59() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 250, 63], OperandSize::Qword)
}

#[test]
fn cmp_60() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 58, 45], OperandSize::Qword)
}

#[test]
fn cmp_61() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 251, 127], OperandSize::Qword)
}

#[test]
fn cmp_62() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1832637777, Some(OperandSize::Byte), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 188, 71, 81, 213, 59, 109, 35], OperandSize::Qword)
}

#[test]
fn cmp_63() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(Literal16(12525)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 250, 237, 48], OperandSize::Word)
}

#[test]
fn cmp_64() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(BP, 21526, Some(OperandSize::Word), None)), operand2: Some(Literal16(10696)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 190, 22, 84, 200, 41], OperandSize::Word)
}

#[test]
fn cmp_65() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(Literal16(25023)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 254, 191, 97], OperandSize::Dword)
}

#[test]
fn cmp_66() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1412706999, Some(OperandSize::Word), None)), operand2: Some(Literal16(12248)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 60, 253, 183, 50, 52, 84, 216, 47], OperandSize::Dword)
}

#[test]
fn cmp_67() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Literal16(20265)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 252, 41, 79], OperandSize::Qword)
}

#[test]
fn cmp_68() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Word), None)), operand2: Some(Literal16(22515)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 60, 182, 243, 87], OperandSize::Qword)
}

#[test]
fn cmp_69() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDI)), operand2: Some(Literal32(400881813)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 255, 149, 248, 228, 23], OperandSize::Word)
}

#[test]
fn cmp_70() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(DI, 194, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1844142249)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 189, 194, 0, 169, 96, 235, 109], OperandSize::Word)
}

#[test]
fn cmp_71() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1546118597)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 254, 197, 229, 39, 92], OperandSize::Dword)
}

#[test]
fn cmp_72() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(408215373)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 63, 77, 223, 84, 24], OperandSize::Dword)
}

#[test]
fn cmp_73() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(Literal32(862659448)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 252, 120, 35, 107, 51], OperandSize::Qword)
}

#[test]
fn cmp_74() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1682233483, Some(OperandSize::Dword), None)), operand2: Some(Literal32(834861127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 188, 82, 139, 216, 68, 100, 71, 248, 194, 49], OperandSize::Qword)
}

#[test]
fn cmp_75() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RSP)), operand2: Some(Literal32(1618865364)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 252, 212, 236, 125, 96], OperandSize::Qword)
}

#[test]
fn cmp_76() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(RSI, 1871357047, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1884395498)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 190, 119, 164, 138, 111, 234, 151, 81, 112], OperandSize::Qword)
}

#[test]
fn cmp_77() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CX)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 249, 77], OperandSize::Word)
}

#[test]
fn cmp_78() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(DI, 12616, Some(OperandSize::Word), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 189, 72, 49, 71], OperandSize::Word)
}

#[test]
fn cmp_79() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DI)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 255, 85], OperandSize::Dword)
}

#[test]
fn cmp_80() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1446635994, Some(OperandSize::Word), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 60, 69, 218, 233, 57, 86, 110], OperandSize::Dword)
}

#[test]
fn cmp_81() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 252, 41], OperandSize::Qword)
}

#[test]
fn cmp_82() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 60, 122, 7], OperandSize::Qword)
}

#[test]
fn cmp_83() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 254, 57], OperandSize::Word)
}

#[test]
fn cmp_84() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 213, Some(OperandSize::Dword), None)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 185, 213, 0, 125], OperandSize::Word)
}

#[test]
fn cmp_85() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 251, 20], OperandSize::Dword)
}

#[test]
fn cmp_86() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 672628825, Some(OperandSize::Dword), None)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 188, 218, 89, 128, 23, 40, 80], OperandSize::Dword)
}

#[test]
fn cmp_87() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBP)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 253, 36], OperandSize::Qword)
}

#[test]
fn cmp_88() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(RCX, 160885891, Some(OperandSize::Dword), None)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 185, 131, 236, 150, 9, 40], OperandSize::Qword)
}

#[test]
fn cmp_89() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RDI)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 255, 0], OperandSize::Qword)
}

#[test]
fn cmp_90() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 60, 207, 34], OperandSize::Qword)
}

