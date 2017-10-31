use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 203], OperandSize::Word)
}

#[test]
fn sub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 16488, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 153, 104, 64], OperandSize::Word)
}

#[test]
fn sub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 203], OperandSize::Dword)
}

#[test]
fn sub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 762921112, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 140, 214, 152, 64, 121, 45], OperandSize::Dword)
}

#[test]
fn sub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 219], OperandSize::Qword)
}

#[test]
fn sub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 24], OperandSize::Qword)
}

#[test]
fn sub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 210], OperandSize::Qword)
}

#[test]
fn sub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 20, 200], OperandSize::Qword)
}

#[test]
fn sub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 243], OperandSize::Word)
}

#[test]
fn sub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 55], OperandSize::Word)
}

#[test]
fn sub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 225], OperandSize::Dword)
}

#[test]
fn sub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 20, 240], OperandSize::Dword)
}

#[test]
fn sub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 247], OperandSize::Qword)
}

#[test]
fn sub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1090226502, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 52, 197, 70, 137, 251, 64], OperandSize::Qword)
}

#[test]
fn sub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 227], OperandSize::Word)
}

#[test]
fn sub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 68, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 74, 68], OperandSize::Word)
}

#[test]
fn sub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 226], OperandSize::Dword)
}

#[test]
fn sub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 75545664, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 156, 81, 64, 188, 128, 4], OperandSize::Dword)
}

#[test]
fn sub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 249], OperandSize::Qword)
}

#[test]
fn sub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RDI, 2113441229, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 159, 205, 141, 248, 125], OperandSize::Qword)
}

#[test]
fn sub_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 249], OperandSize::Qword)
}

#[test]
fn sub_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1260786873, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 12, 205, 185, 20, 38, 75], OperandSize::Qword)
}

#[test]
fn sub_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 201], OperandSize::Word)
}

#[test]
fn sub_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(IndirectDisplaced(BX, 21466, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 159, 218, 83], OperandSize::Word)
}

#[test]
fn sub_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 219], OperandSize::Dword)
}

#[test]
fn sub_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 12, 115], OperandSize::Dword)
}

#[test]
fn sub_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 217], OperandSize::Qword)
}

#[test]
fn sub_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1207931775, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 156, 144, 127, 147, 255, 71], OperandSize::Qword)
}

#[test]
fn sub_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 217], OperandSize::Qword)
}

#[test]
fn sub_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 28, 255], OperandSize::Qword)
}

#[test]
fn sub_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 218], OperandSize::Word)
}

#[test]
fn sub_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 27], OperandSize::Word)
}

#[test]
fn sub_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 241], OperandSize::Dword)
}

#[test]
fn sub_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1625741058, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 148, 152, 2, 215, 230, 96], OperandSize::Dword)
}

#[test]
fn sub_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 252], OperandSize::Qword)
}

#[test]
fn sub_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 44, 78], OperandSize::Qword)
}

#[test]
fn sub_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 239], OperandSize::Word)
}

#[test]
fn sub_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 152, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 187, 152, 0], OperandSize::Word)
}

#[test]
fn sub_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 247], OperandSize::Dword)
}

#[test]
fn sub_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 28, 218], OperandSize::Dword)
}

#[test]
fn sub_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 213], OperandSize::Qword)
}

#[test]
fn sub_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 290262309, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 148, 78, 37, 13, 77, 17], OperandSize::Qword)
}

#[test]
fn sub_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 236], OperandSize::Qword)
}

#[test]
fn sub_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 36253974, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 43, 20, 181, 22, 49, 41, 2], OperandSize::Qword)
}

#[test]
fn sub_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 21], OperandSize::Word)
}

#[test]
fn sub_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 122], OperandSize::Dword)
}

#[test]
fn sub_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 69], OperandSize::Qword)
}

#[test]
fn sub_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(14024)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 200, 54], OperandSize::Word)
}

#[test]
fn sub_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(3064)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 248, 11], OperandSize::Dword)
}

#[test]
fn sub_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(29624)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 184, 115], OperandSize::Qword)
}

#[test]
fn sub_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(495461107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 243, 34, 136, 29], OperandSize::Word)
}

#[test]
fn sub_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1168294251)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 107, 193, 162, 69], OperandSize::Dword)
}

#[test]
fn sub_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1717308902)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 230, 13, 92, 102], OperandSize::Qword)
}

#[test]
fn sub_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RAX)), operand2: Some(Literal32(536932320)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 45, 224, 239, 0, 32], OperandSize::Qword)
}

#[test]
fn sub_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 235, 17], OperandSize::Word)
}

#[test]
fn sub_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 29131, Some(OperandSize::Byte), None)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 169, 203, 113, 3], OperandSize::Word)
}

#[test]
fn sub_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 235, 0], OperandSize::Dword)
}

#[test]
fn sub_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 44, 211, 52], OperandSize::Dword)
}

#[test]
fn sub_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 234, 80], OperandSize::Qword)
}

#[test]
fn sub_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RCX, 1093784549, Some(OperandSize::Byte), None)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 169, 229, 211, 49, 65, 85], OperandSize::Qword)
}

#[test]
fn sub_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 233, 6], OperandSize::Qword)
}

#[test]
fn sub_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 2055194328, Some(OperandSize::Byte), None)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 172, 153, 216, 198, 127, 122, 125], OperandSize::Qword)
}

#[test]
fn sub_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CX)), operand2: Some(Literal16(21620)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 233, 116, 84], OperandSize::Word)
}

#[test]
fn sub_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal16(25131)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 42, 43, 98], OperandSize::Word)
}

#[test]
fn sub_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Literal16(3616)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 234, 32, 14], OperandSize::Dword)
}

#[test]
fn sub_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(ECX, Two, 871729863, Some(OperandSize::Word), None)), operand2: Some(Literal16(27129)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 44, 77, 199, 138, 245, 51, 249, 105], OperandSize::Dword)
}

#[test]
fn sub_67() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Literal16(26394)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 235, 26, 103], OperandSize::Qword)
}

#[test]
fn sub_68() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Word), None)), operand2: Some(Literal16(23664)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 44, 176, 112, 92], OperandSize::Qword)
}

#[test]
fn sub_69() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1273805536)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 237, 224, 186, 236, 75], OperandSize::Word)
}

#[test]
fn sub_70() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1550918722)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 44, 66, 36, 113, 92], OperandSize::Word)
}

#[test]
fn sub_71() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Literal32(435834267)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 235, 155, 77, 250, 25], OperandSize::Dword)
}

#[test]
fn sub_72() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 751815688, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1367907118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 172, 143, 8, 204, 207, 44, 46, 155, 136, 81], OperandSize::Dword)
}

#[test]
fn sub_73() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ECX)), operand2: Some(Literal32(1772698898)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 233, 18, 61, 169, 105], OperandSize::Qword)
}

#[test]
fn sub_74() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RDX, Four, 549006321, Some(OperandSize::Dword), None)), operand2: Some(Literal32(221255116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 44, 149, 241, 43, 185, 32, 204, 21, 48, 13], OperandSize::Qword)
}

#[test]
fn sub_75() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RBX)), operand2: Some(Literal32(1336121406)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 235, 62, 152, 163, 79], OperandSize::Qword)
}

#[test]
fn sub_76() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 812781290, Some(OperandSize::Qword), None)), operand2: Some(Literal32(520379511)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 172, 94, 234, 14, 114, 48, 119, 92, 4, 31], OperandSize::Qword)
}

#[test]
fn sub_77() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 235, 25], OperandSize::Word)
}

#[test]
fn sub_78() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(SI, 2121, Some(OperandSize::Word), None)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 172, 73, 8, 49], OperandSize::Word)
}

#[test]
fn sub_79() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 235, 92], OperandSize::Dword)
}

#[test]
fn sub_80() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1121042372, Some(OperandSize::Word), None)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 172, 115, 196, 191, 209, 66, 70], OperandSize::Dword)
}

#[test]
fn sub_81() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 235, 15], OperandSize::Qword)
}

#[test]
fn sub_82() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 44, 185, 103], OperandSize::Qword)
}

#[test]
fn sub_83() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 235, 92], OperandSize::Word)
}

#[test]
fn sub_84() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(BX, 140, Some(OperandSize::Dword), None)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 175, 140, 0, 78], OperandSize::Word)
}

#[test]
fn sub_85() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 239, 20], OperandSize::Dword)
}

#[test]
fn sub_86() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(ESI, Four, 997584499, Some(OperandSize::Dword), None)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 44, 181, 115, 238, 117, 59, 108], OperandSize::Dword)
}

#[test]
fn sub_87() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ECX)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 233, 38], OperandSize::Qword)
}

#[test]
fn sub_88() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RSI, Four, 2128203510, Some(OperandSize::Dword), None)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 44, 181, 246, 206, 217, 126, 73], OperandSize::Qword)
}

#[test]
fn sub_89() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RBX)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 235, 95], OperandSize::Qword)
}

#[test]
fn sub_90() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RCX, Two, 8340303, Some(OperandSize::Qword), None)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 44, 77, 79, 67, 127, 0, 40], OperandSize::Qword)
}

