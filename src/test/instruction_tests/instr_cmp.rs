use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 217], OperandSize::Word)
}

#[test]
fn cmp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 9], OperandSize::Word)
}

#[test]
fn cmp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 209], OperandSize::Dword)
}

#[test]
fn cmp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1837529092, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 148, 131, 4, 120, 134, 109], OperandSize::Dword)
}

#[test]
fn cmp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 201], OperandSize::Qword)
}

#[test]
fn cmp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RDI, Two, 112484966, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 12, 125, 102, 98, 180, 6], OperandSize::Qword)
}

#[test]
fn cmp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 219], OperandSize::Qword)
}

#[test]
fn cmp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 726972610, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 148, 203, 194, 184, 84, 43], OperandSize::Qword)
}

#[test]
fn cmp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 229], OperandSize::Word)
}

#[test]
fn cmp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(SI, 140, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 180, 140, 0], OperandSize::Word)
}

#[test]
fn cmp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 242], OperandSize::Dword)
}

#[test]
fn cmp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 786380717, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 148, 136, 173, 55, 223, 46], OperandSize::Dword)
}

#[test]
fn cmp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 207], OperandSize::Qword)
}

#[test]
fn cmp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1192753728, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 188, 128, 64, 250, 23, 71], OperandSize::Qword)
}

#[test]
fn cmp_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 233], OperandSize::Word)
}

#[test]
fn cmp_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Memory(19069, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 30, 125, 74], OperandSize::Word)
}

#[test]
fn cmp_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 202], OperandSize::Dword)
}

#[test]
fn cmp_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 28, 130], OperandSize::Dword)
}

#[test]
fn cmp_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 227], OperandSize::Qword)
}

#[test]
fn cmp_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1286702314, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 20, 133, 234, 132, 177, 76], OperandSize::Qword)
}

#[test]
fn cmp_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 214], OperandSize::Qword)
}

#[test]
fn cmp_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 36, 178], OperandSize::Qword)
}

#[test]
fn cmp_23() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 203], OperandSize::Word)
}

#[test]
fn cmp_24() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 10159, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 136, 175, 39], OperandSize::Word)
}

#[test]
fn cmp_25() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 209], OperandSize::Dword)
}

#[test]
fn cmp_26() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(ECX, 1000884484, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 145, 4, 73, 168, 59], OperandSize::Dword)
}

#[test]
fn cmp_27() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 210], OperandSize::Qword)
}

#[test]
fn cmp_28() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(RAX, 13972836, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 144, 100, 53, 213, 0], OperandSize::Qword)
}

#[test]
fn cmp_29() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 210], OperandSize::Qword)
}

#[test]
fn cmp_30() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(IndirectDisplaced(RBX, 1928266380, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 155, 140, 2, 239, 114], OperandSize::Qword)
}

#[test]
fn cmp_31() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 252], OperandSize::Word)
}

#[test]
fn cmp_32() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 120, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 83, 120], OperandSize::Word)
}

#[test]
fn cmp_33() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 234], OperandSize::Dword)
}

#[test]
fn cmp_34() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(ESI, 1494292690, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 166, 210, 24, 17, 89], OperandSize::Dword)
}

#[test]
fn cmp_35() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 244], OperandSize::Qword)
}

#[test]
fn cmp_36() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 1168608632, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 188, 119, 120, 141, 167, 69], OperandSize::Qword)
}

#[test]
fn cmp_37() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 209], OperandSize::Word)
}

#[test]
fn cmp_38() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 81, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 74, 81], OperandSize::Word)
}

#[test]
fn cmp_39() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 210], OperandSize::Dword)
}

#[test]
fn cmp_40() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(EAX, 248484169, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 144, 73, 145, 207, 14], OperandSize::Dword)
}

#[test]
fn cmp_41() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 202], OperandSize::Qword)
}

#[test]
fn cmp_42() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RSI, 480091228, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 190, 92, 156, 157, 28], OperandSize::Qword)
}

#[test]
fn cmp_43() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 212], OperandSize::Qword)
}

#[test]
fn cmp_44() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1057066859, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 59, 188, 223, 107, 143, 1, 63], OperandSize::Qword)
}

#[test]
fn cmp_45() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 112], OperandSize::Word)
}

#[test]
fn cmp_46() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 116], OperandSize::Dword)
}

#[test]
fn cmp_47() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 114], OperandSize::Qword)
}

#[test]
fn cmp_48() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(8162)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 226, 31], OperandSize::Word)
}

#[test]
fn cmp_49() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(32282)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 26, 126], OperandSize::Dword)
}

#[test]
fn cmp_50() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(27212)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 76, 106], OperandSize::Qword)
}

#[test]
fn cmp_51() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1498965897)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 137, 103, 88, 89], OperandSize::Word)
}

#[test]
fn cmp_52() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1819045257)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 137, 109, 108, 108], OperandSize::Dword)
}

#[test]
fn cmp_53() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1816723876)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 164, 1, 73, 108], OperandSize::Qword)
}

#[test]
fn cmp_54() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RAX)), operand2: Some(Literal32(94509385)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 61, 73, 25, 162, 5], OperandSize::Qword)
}

#[test]
fn cmp_55() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 251, 62], OperandSize::Word)
}

#[test]
fn cmp_56() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(BX, 4107, Some(OperandSize::Byte), None)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 191, 11, 16, 38], OperandSize::Word)
}

#[test]
fn cmp_57() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 250, 121], OperandSize::Dword)
}

#[test]
fn cmp_58() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(EAX, Two, 2085132231, Some(OperandSize::Byte), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 60, 69, 199, 151, 72, 124, 115], OperandSize::Dword)
}

#[test]
fn cmp_59() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 250, 23], OperandSize::Qword)
}

#[test]
fn cmp_60() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 239021768, Some(OperandSize::Byte), None)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 188, 142, 200, 46, 63, 14, 63], OperandSize::Qword)
}

#[test]
fn cmp_61() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Literal8(124)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 251, 124], OperandSize::Qword)
}

#[test]
fn cmp_62() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1475002008, Some(OperandSize::Byte), None)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 188, 208, 152, 190, 234, 87, 15], OperandSize::Qword)
}

#[test]
fn cmp_63() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(Literal16(2978)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 250, 162, 11], OperandSize::Word)
}

#[test]
fn cmp_64() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(DI, 31056, Some(OperandSize::Word), None)), operand2: Some(Literal16(4639)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 189, 80, 121, 31, 18], OperandSize::Word)
}

#[test]
fn cmp_65() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BP)), operand2: Some(Literal16(3239)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 253, 167, 12], OperandSize::Dword)
}

#[test]
fn cmp_66() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal16(18968)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 59, 24, 74], OperandSize::Dword)
}

#[test]
fn cmp_67() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(Literal16(8968)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 250, 8, 35], OperandSize::Qword)
}

#[test]
fn cmp_68() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1333554424, Some(OperandSize::Word), None)), operand2: Some(Literal16(31734)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 60, 189, 248, 108, 124, 79, 246, 123], OperandSize::Qword)
}

#[test]
fn cmp_69() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(Literal32(1712900044)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 251, 204, 199, 24, 102], OperandSize::Word)
}

#[test]
fn cmp_70() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(DI, 30236, Some(OperandSize::Dword), None)), operand2: Some(Literal32(653466922)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 189, 28, 118, 42, 29, 243, 38], OperandSize::Word)
}

#[test]
fn cmp_71() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBP)), operand2: Some(Literal32(254038878)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 253, 94, 83, 36, 15], OperandSize::Dword)
}

#[test]
fn cmp_72() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(EDI, 1763537085, Some(OperandSize::Dword), None)), operand2: Some(Literal32(469580465)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 191, 189, 112, 29, 105, 177, 58, 253, 27], OperandSize::Dword)
}

#[test]
fn cmp_73() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1737242551)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 254, 183, 55, 140, 103], OperandSize::Qword)
}

#[test]
fn cmp_74() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal32(742766080)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 60, 139, 0, 182, 69, 44], OperandSize::Qword)
}

#[test]
fn cmp_75() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RBP)), operand2: Some(Literal32(967685151)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 253, 31, 180, 173, 57], OperandSize::Qword)
}

#[test]
fn cmp_76() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 543736563, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1065807492)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 188, 191, 243, 194, 104, 32, 132, 238, 134, 63], OperandSize::Qword)
}

#[test]
fn cmp_77() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 254, 100], OperandSize::Word)
}

#[test]
fn cmp_78() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 57, 43], OperandSize::Word)
}

#[test]
fn cmp_79() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CX)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 249, 16], OperandSize::Dword)
}

#[test]
fn cmp_80() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1592417160, Some(OperandSize::Word), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 60, 117, 136, 91, 234, 94, 71], OperandSize::Dword)
}

#[test]
fn cmp_81() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BX)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 251, 125], OperandSize::Qword)
}

#[test]
fn cmp_82() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 60, 249, 35], OperandSize::Qword)
}

#[test]
fn cmp_83() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 250, 69], OperandSize::Word)
}

#[test]
fn cmp_84() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 60, Some(OperandSize::Dword), None)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 122, 60, 67], OperandSize::Word)
}

#[test]
fn cmp_85() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 252, 122], OperandSize::Dword)
}

#[test]
fn cmp_86() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(EAX, 800374267, Some(OperandSize::Dword), None)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 184, 251, 189, 180, 47, 46], OperandSize::Dword)
}

#[test]
fn cmp_87() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 249, 123], OperandSize::Qword)
}

#[test]
fn cmp_88() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(RDI, 203617484, Some(OperandSize::Dword), None)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 191, 204, 244, 34, 12, 85], OperandSize::Qword)
}

#[test]
fn cmp_89() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RSI)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 254, 72], OperandSize::Qword)
}

#[test]
fn cmp_90() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1852643823, Some(OperandSize::Qword), None)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 188, 155, 239, 25, 109, 110, 79], OperandSize::Qword)
}

