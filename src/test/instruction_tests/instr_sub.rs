use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 209], OperandSize::Word)
}

#[test]
fn sub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 90, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 72, 90], OperandSize::Word)
}

#[test]
fn sub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 203], OperandSize::Dword)
}

#[test]
fn sub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(EDX, 1850273902, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 146, 110, 240, 72, 110], OperandSize::Dword)
}

#[test]
fn sub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 210], OperandSize::Qword)
}

#[test]
fn sub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 30], OperandSize::Qword)
}

#[test]
fn sub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 210], OperandSize::Qword)
}

#[test]
fn sub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 445306536, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 156, 208, 168, 214, 138, 26], OperandSize::Qword)
}

#[test]
fn sub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 254], OperandSize::Word)
}

#[test]
fn sub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 18], OperandSize::Word)
}

#[test]
fn sub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 236], OperandSize::Dword)
}

#[test]
fn sub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(EBX, 873815981, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 163, 173, 95, 21, 52], OperandSize::Dword)
}

#[test]
fn sub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 221], OperandSize::Qword)
}

#[test]
fn sub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 61534226, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 60, 245, 18, 240, 170, 3], OperandSize::Qword)
}

#[test]
fn sub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 233], OperandSize::Word)
}

#[test]
fn sub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 16], OperandSize::Word)
}

#[test]
fn sub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 211], OperandSize::Dword)
}

#[test]
fn sub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 56], OperandSize::Dword)
}

#[test]
fn sub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 242], OperandSize::Qword)
}

#[test]
fn sub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1105457318, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 28, 221, 166, 240, 227, 65], OperandSize::Qword)
}

#[test]
fn sub_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RBP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 205], OperandSize::Qword)
}

#[test]
fn sub_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RBX, 2039808520, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 139, 8, 2, 149, 121], OperandSize::Qword)
}

#[test]
fn sub_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 218], OperandSize::Word)
}

#[test]
fn sub_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 27059, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 147, 179, 105], OperandSize::Word)
}

#[test]
fn sub_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 211], OperandSize::Dword)
}

#[test]
fn sub_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 28, 206], OperandSize::Dword)
}

#[test]
fn sub_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 201], OperandSize::Qword)
}

#[test]
fn sub_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 28, 249], OperandSize::Qword)
}

#[test]
fn sub_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 211], OperandSize::Qword)
}

#[test]
fn sub_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(IndirectDisplaced(RDX, 354169291, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 154, 203, 49, 28, 21], OperandSize::Qword)
}

#[test]
fn sub_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 205], OperandSize::Word)
}

#[test]
fn sub_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 27], OperandSize::Word)
}

#[test]
fn sub_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 235], OperandSize::Dword)
}

#[test]
fn sub_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CX)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 8], OperandSize::Dword)
}

#[test]
fn sub_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 253], OperandSize::Qword)
}

#[test]
fn sub_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 425526225, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 60, 253, 209, 3, 93, 25], OperandSize::Qword)
}

#[test]
fn sub_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 249], OperandSize::Word)
}

#[test]
fn sub_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 33], OperandSize::Word)
}

#[test]
fn sub_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 252], OperandSize::Dword)
}

#[test]
fn sub_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1143398059, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 140, 183, 171, 222, 38, 68], OperandSize::Dword)
}

#[test]
fn sub_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 214], OperandSize::Qword)
}

#[test]
fn sub_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 80620660, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 60, 133, 116, 44, 206, 4], OperandSize::Qword)
}

#[test]
fn sub_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RCX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 201], OperandSize::Qword)
}

#[test]
fn sub_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 43, 12, 190], OperandSize::Qword)
}

#[test]
fn sub_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 16], OperandSize::Word)
}

#[test]
fn sub_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 115], OperandSize::Dword)
}

#[test]
fn sub_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 116], OperandSize::Qword)
}

#[test]
fn sub_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(21362)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 114, 83], OperandSize::Word)
}

#[test]
fn sub_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(15366)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 6, 60], OperandSize::Dword)
}

#[test]
fn sub_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(2297)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 249, 8], OperandSize::Qword)
}

#[test]
fn sub_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1123440308)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 180, 86, 246, 66], OperandSize::Word)
}

#[test]
fn sub_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(365504912)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 144, 41, 201, 21], OperandSize::Dword)
}

#[test]
fn sub_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(642733085)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 29, 84, 79, 38], OperandSize::Qword)
}

#[test]
fn sub_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RAX)), operand2: Some(Literal32(787368136)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 45, 200, 72, 238, 46], OperandSize::Qword)
}

#[test]
fn sub_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 233, 3], OperandSize::Word)
}

#[test]
fn sub_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(DI, 246, Some(OperandSize::Byte), None)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 173, 246, 0, 122], OperandSize::Word)
}

#[test]
fn sub_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 235, 100], OperandSize::Dword)
}

#[test]
fn sub_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 47, 30], OperandSize::Dword)
}

#[test]
fn sub_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 234, 125], OperandSize::Qword)
}

#[test]
fn sub_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 44, 113, 49], OperandSize::Qword)
}

#[test]
fn sub_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 235, 114], OperandSize::Qword)
}

#[test]
fn sub_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 1270488867, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 172, 214, 35, 31, 186, 75, 1], OperandSize::Qword)
}

#[test]
fn sub_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Literal16(30962)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 235, 242, 120], OperandSize::Word)
}

#[test]
fn sub_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(BX, 15, Some(OperandSize::Word), None)), operand2: Some(Literal16(11934)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 111, 15, 158, 46], OperandSize::Word)
}

#[test]
fn sub_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Literal16(12909)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 235, 109, 50], OperandSize::Dword)
}

#[test]
fn sub_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(ECX, Two, 79812937, Some(OperandSize::Word), None)), operand2: Some(Literal16(25488)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 44, 77, 73, 217, 193, 4, 144, 99], OperandSize::Dword)
}

#[test]
fn sub_67() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CX)), operand2: Some(Literal16(19635)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 233, 179, 76], OperandSize::Qword)
}

#[test]
fn sub_68() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RSI, Four, 69652899, Some(OperandSize::Word), None)), operand2: Some(Literal16(2604)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 44, 181, 163, 209, 38, 4, 44, 10], OperandSize::Qword)
}

#[test]
fn sub_69() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(Literal32(173345055)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 237, 31, 9, 85, 10], OperandSize::Word)
}

#[test]
fn sub_70() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(SI, 24116, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1199676432)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 172, 52, 94, 16, 156, 129, 71], OperandSize::Word)
}

#[test]
fn sub_71() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Literal32(160201787)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 235, 59, 124, 140, 9], OperandSize::Dword)
}

#[test]
fn sub_72() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(EBX, Four, 2097591305, Some(OperandSize::Dword), None)), operand2: Some(Literal32(2125472894)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 44, 157, 9, 180, 6, 125, 126, 36, 176, 126], OperandSize::Dword)
}

#[test]
fn sub_73() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Literal32(344336324)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 238, 196, 39, 134, 20], OperandSize::Qword)
}

#[test]
fn sub_74() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RCX, 1691187855, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1539556655)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 169, 143, 122, 205, 100, 47, 197, 195, 91], OperandSize::Qword)
}

#[test]
fn sub_75() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RDI)), operand2: Some(Literal32(487834970)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 239, 90, 197, 19, 29], OperandSize::Qword)
}

#[test]
fn sub_76() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Qword), None)), operand2: Some(Literal32(973159678)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 44, 184, 254, 60, 1, 58], OperandSize::Qword)
}

#[test]
fn sub_77() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 234, 36], OperandSize::Word)
}

#[test]
fn sub_78() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 20747, Some(OperandSize::Word), None)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 169, 11, 81, 9], OperandSize::Word)
}

#[test]
fn sub_79() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 239, 55], OperandSize::Dword)
}

#[test]
fn sub_80() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(ESI, 639445320, Some(OperandSize::Word), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 174, 72, 41, 29, 38, 35], OperandSize::Dword)
}

#[test]
fn sub_81() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BP)), operand2: Some(Literal8(13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 237, 13], OperandSize::Qword)
}

#[test]
fn sub_82() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1772210317, Some(OperandSize::Word), None)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 44, 85, 141, 200, 161, 105, 39], OperandSize::Qword)
}

#[test]
fn sub_83() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 237, 87], OperandSize::Word)
}

#[test]
fn sub_84() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(DI, 129, Some(OperandSize::Dword), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 173, 129, 0, 105], OperandSize::Word)
}

#[test]
fn sub_85() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 237, 57], OperandSize::Dword)
}

#[test]
fn sub_86() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 47, 80], OperandSize::Dword)
}

#[test]
fn sub_87() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 235, 117], OperandSize::Qword)
}

#[test]
fn sub_88() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RDI, Two, 929059857, Some(OperandSize::Dword), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 44, 125, 17, 84, 96, 55, 113], OperandSize::Qword)
}

#[test]
fn sub_89() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RBX)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 235, 51], OperandSize::Qword)
}

#[test]
fn sub_90() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 44, 82, 72], OperandSize::Qword)
}

