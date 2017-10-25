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
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(BX, 224, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 143, 224, 0], OperandSize::Word)
}

#[test]
fn sub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 210], OperandSize::Dword)
}

#[test]
fn sub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 8], OperandSize::Dword)
}

#[test]
fn sub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 218], OperandSize::Qword)
}

#[test]
fn sub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1236465461, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 20, 253, 53, 247, 178, 73], OperandSize::Qword)
}

#[test]
fn sub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 201], OperandSize::Qword)
}

#[test]
fn sub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 9], OperandSize::Qword)
}

#[test]
fn sub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 219], OperandSize::Word)
}

#[test]
fn sub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Memory(8391, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 30, 199, 32], OperandSize::Word)
}

#[test]
fn sub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 234], OperandSize::Dword)
}

#[test]
fn sub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 23], OperandSize::Dword)
}

#[test]
fn sub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 230], OperandSize::Qword)
}

#[test]
fn sub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 978508054, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 60, 205, 22, 217, 82, 58], OperandSize::Qword)
}

#[test]
fn sub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 229], OperandSize::Word)
}

#[test]
fn sub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(DI, 4, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 117, 4], OperandSize::Word)
}

#[test]
fn sub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 228], OperandSize::Dword)
}

#[test]
fn sub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 52, 192], OperandSize::Dword)
}

#[test]
fn sub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 207], OperandSize::Qword)
}

#[test]
fn sub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 20, 214], OperandSize::Qword)
}

#[test]
fn sub_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 251], OperandSize::Qword)
}

#[test]
fn sub_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RDX, 1632534378, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 146, 106, 127, 78, 97], OperandSize::Qword)
}

#[test]
fn sub_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 217], OperandSize::Word)
}

#[test]
fn sub_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(BX, 63, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 79, 63], OperandSize::Word)
}

#[test]
fn sub_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 218], OperandSize::Dword)
}

#[test]
fn sub_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 12, 187], OperandSize::Dword)
}

#[test]
fn sub_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 203], OperandSize::Qword)
}

#[test]
fn sub_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 14], OperandSize::Qword)
}

#[test]
fn sub_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 209], OperandSize::Qword)
}

#[test]
fn sub_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 12, 127], OperandSize::Qword)
}

#[test]
fn sub_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 218], OperandSize::Word)
}

#[test]
fn sub_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(DI, 29943, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 189, 247, 116], OperandSize::Word)
}

#[test]
fn sub_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 246], OperandSize::Dword)
}

#[test]
fn sub_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 240972132, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 52, 125, 100, 241, 92, 14], OperandSize::Dword)
}

#[test]
fn sub_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 228], OperandSize::Qword)
}

#[test]
fn sub_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CX)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 9], OperandSize::Qword)
}

#[test]
fn sub_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 223], OperandSize::Word)
}

#[test]
fn sub_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(BX, 17701, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 159, 37, 69], OperandSize::Word)
}

#[test]
fn sub_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 254], OperandSize::Dword)
}

#[test]
fn sub_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 44, 223], OperandSize::Dword)
}

#[test]
fn sub_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 242], OperandSize::Qword)
}

#[test]
fn sub_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 237540885, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 44, 141, 21, 150, 40, 14], OperandSize::Qword)
}

#[test]
fn sub_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 220], OperandSize::Qword)
}

#[test]
fn sub_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RBP)), operand2: Some(IndirectDisplaced(RSI, 1676911769, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 43, 174, 153, 164, 243, 99], OperandSize::Qword)
}

#[test]
fn sub_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 23], OperandSize::Word)
}

#[test]
fn sub_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 89], OperandSize::Dword)
}

#[test]
fn sub_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 27], OperandSize::Qword)
}

#[test]
fn sub_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(26249)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 137, 102], OperandSize::Word)
}

#[test]
fn sub_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(10189)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 205, 39], OperandSize::Dword)
}

#[test]
fn sub_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(4326)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 230, 16], OperandSize::Qword)
}

#[test]
fn sub_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(279890508)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 76, 202, 174, 16], OperandSize::Word)
}

#[test]
fn sub_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(877011609)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 153, 34, 70, 52], OperandSize::Dword)
}

#[test]
fn sub_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(140973350)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 38, 21, 103, 8], OperandSize::Qword)
}

#[test]
fn sub_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1275655177)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 45, 9, 244, 8, 76], OperandSize::Qword)
}

#[test]
fn sub_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 234, 59], OperandSize::Word)
}

#[test]
fn sub_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 47, 98], OperandSize::Word)
}

#[test]
fn sub_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 233, 34], OperandSize::Dword)
}

#[test]
fn sub_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 40, 18], OperandSize::Dword)
}

#[test]
fn sub_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 235, 38], OperandSize::Qword)
}

#[test]
fn sub_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 44, 192, 2], OperandSize::Qword)
}

#[test]
fn sub_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 235, 11], OperandSize::Qword)
}

#[test]
fn sub_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RCX, Two, 484921618, Some(OperandSize::Byte), None)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 44, 77, 18, 81, 231, 28, 61], OperandSize::Qword)
}

#[test]
fn sub_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Literal16(27470)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 234, 78, 107], OperandSize::Word)
}

#[test]
fn sub_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 25947, Some(OperandSize::Word), None)), operand2: Some(Literal16(8586)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 168, 91, 101, 138, 33], OperandSize::Word)
}

#[test]
fn sub_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(Literal16(26535)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 239, 167, 103], OperandSize::Dword)
}

#[test]
fn sub_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal16(27147)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 43, 11, 106], OperandSize::Dword)
}

#[test]
fn sub_67() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CX)), operand2: Some(Literal16(24918)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 233, 86, 97], OperandSize::Qword)
}

#[test]
fn sub_68() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1377445078, Some(OperandSize::Word), None)), operand2: Some(Literal16(989)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 44, 85, 214, 36, 26, 82, 221, 3], OperandSize::Qword)
}

#[test]
fn sub_69() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Literal32(834406157)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 238, 13, 7, 188, 49], OperandSize::Word)
}

#[test]
fn sub_70() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(DI, 150, Some(OperandSize::Dword), None)), operand2: Some(Literal32(30326040)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 173, 150, 0, 24, 189, 206, 1], OperandSize::Word)
}

#[test]
fn sub_71() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Literal32(50913892)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 238, 100, 226, 8, 3], OperandSize::Dword)
}

#[test]
fn sub_72() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1106162509, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1992302844)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 44, 93, 77, 179, 238, 65, 252, 32, 192, 118], OperandSize::Dword)
}

#[test]
fn sub_73() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBP)), operand2: Some(Literal32(81637852)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 237, 220, 177, 221, 4], OperandSize::Qword)
}

#[test]
fn sub_74() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1846345976)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 40, 248, 0, 13, 110], OperandSize::Qword)
}

#[test]
fn sub_75() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RCX)), operand2: Some(Literal32(1790556318)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 233, 158, 184, 185, 106], OperandSize::Qword)
}

#[test]
fn sub_76() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1289203744)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 44, 139, 32, 176, 215, 76], OperandSize::Qword)
}

#[test]
fn sub_77() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BP)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 237, 29], OperandSize::Word)
}

#[test]
fn sub_78() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 42, 105], OperandSize::Word)
}

#[test]
fn sub_79() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 234, 115], OperandSize::Dword)
}

#[test]
fn sub_80() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 66806384, Some(OperandSize::Word), None)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 172, 182, 112, 98, 251, 3, 18], OperandSize::Dword)
}

#[test]
fn sub_81() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SI)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 238, 49], OperandSize::Qword)
}

#[test]
fn sub_82() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 249229045, Some(OperandSize::Word), None)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 172, 145, 245, 238, 218, 14, 125], OperandSize::Qword)
}

#[test]
fn sub_83() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 238, 71], OperandSize::Word)
}

#[test]
fn sub_84() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(DI, 11835, Some(OperandSize::Dword), None)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 173, 59, 46, 122], OperandSize::Word)
}

#[test]
fn sub_85() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 235, 103], OperandSize::Dword)
}

#[test]
fn sub_86() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 40, 77], OperandSize::Dword)
}

#[test]
fn sub_87() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDX)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 234, 81], OperandSize::Qword)
}

#[test]
fn sub_88() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RBX, 1548627659, Some(OperandSize::Dword), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 171, 203, 46, 78, 92, 75], OperandSize::Qword)
}

#[test]
fn sub_89() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RBP)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 237, 59], OperandSize::Qword)
}

#[test]
fn sub_90() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RSI, 975837091, Some(OperandSize::Qword), None)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 174, 163, 23, 42, 58, 20], OperandSize::Qword)
}

