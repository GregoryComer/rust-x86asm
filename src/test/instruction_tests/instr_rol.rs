use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rol_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 194, 68], OperandSize::Word)
}

#[test]
fn rol_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(BP, 30288, Some(OperandSize::Byte), None)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 134, 80, 118, 97], OperandSize::Word)
}

#[test]
fn rol_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 195, 26], OperandSize::Dword)
}

#[test]
fn rol_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 31671348, Some(OperandSize::Byte), None)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 132, 184, 52, 68, 227, 1, 50], OperandSize::Dword)
}

#[test]
fn rol_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 194, 98], OperandSize::Qword)
}

#[test]
fn rol_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 6, 126], OperandSize::Qword)
}

#[test]
fn rol_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 195, 67], OperandSize::Qword)
}

#[test]
fn rol_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1400189764, Some(OperandSize::Byte), None)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 132, 89, 68, 51, 117, 83, 65], OperandSize::Qword)
}

#[test]
fn rol_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BP)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 197, 57], OperandSize::Word)
}

#[test]
fn rol_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 57, Some(OperandSize::Word), None)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 65, 57, 4], OperandSize::Word)
}

#[test]
fn rol_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DX)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 194, 90], OperandSize::Dword)
}

#[test]
fn rol_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 2, 78], OperandSize::Dword)
}

#[test]
fn rol_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BX)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 195, 101], OperandSize::Qword)
}

#[test]
fn rol_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 4, 201, 82], OperandSize::Qword)
}

#[test]
fn rol_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 197, 53], OperandSize::Word)
}

#[test]
fn rol_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 22804, Some(OperandSize::Dword), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 130, 20, 89, 82], OperandSize::Word)
}

#[test]
fn rol_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 198, 52], OperandSize::Dword)
}

#[test]
fn rol_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 4, 178, 117], OperandSize::Dword)
}

#[test]
fn rol_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 193, 37], OperandSize::Qword)
}

#[test]
fn rol_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 3, 37], OperandSize::Qword)
}

#[test]
fn rol_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RSI)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 198, 123], OperandSize::Qword)
}

#[test]
fn rol_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1000136075, Some(OperandSize::Qword), None)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 132, 194, 139, 221, 156, 59, 4], OperandSize::Qword)
}

#[test]
fn rol_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 193], OperandSize::Word)
}

#[test]
fn rol_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(DI, 41, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 69, 41], OperandSize::Word)
}

#[test]
fn rol_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 194], OperandSize::Dword)
}

#[test]
fn rol_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 2121179892, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 132, 145, 244, 162, 110, 126], OperandSize::Dword)
}

#[test]
fn rol_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 193], OperandSize::Qword)
}

#[test]
fn rol_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1410541998, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 4, 189, 174, 41, 19, 84], OperandSize::Qword)
}

#[test]
fn rol_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 195], OperandSize::Qword)
}

#[test]
fn rol_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1390061053, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 132, 155, 253, 165, 218, 82], OperandSize::Qword)
}

#[test]
fn rol_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 196], OperandSize::Word)
}

#[test]
fn rol_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 16495, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 131, 111, 64], OperandSize::Word)
}

#[test]
fn rol_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 194], OperandSize::Dword)
}

#[test]
fn rol_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1980300603, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 132, 191, 59, 253, 8, 118], OperandSize::Dword)
}

#[test]
fn rol_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 197], OperandSize::Qword)
}

#[test]
fn rol_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 2], OperandSize::Qword)
}

#[test]
fn rol_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 194], OperandSize::Word)
}

#[test]
fn rol_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 1], OperandSize::Word)
}

#[test]
fn rol_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 199], OperandSize::Dword)
}

#[test]
fn rol_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1275421327, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 132, 89, 143, 98, 5, 76], OperandSize::Dword)
}

#[test]
fn rol_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 199], OperandSize::Qword)
}

#[test]
fn rol_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1819533544, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 132, 248, 232, 224, 115, 108], OperandSize::Qword)
}

#[test]
fn rol_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RSP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 196], OperandSize::Qword)
}

#[test]
fn rol_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 2097923855, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 132, 122, 15, 199, 11, 125], OperandSize::Qword)
}

#[test]
fn rol_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 195], OperandSize::Word)
}

#[test]
fn rol_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 20, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 66, 20], OperandSize::Word)
}

#[test]
fn rol_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 195], OperandSize::Dword)
}

#[test]
fn rol_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 4, 215], OperandSize::Dword)
}

#[test]
fn rol_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Qword)
}

#[test]
fn rol_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 4, 119], OperandSize::Qword)
}

#[test]
fn rol_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Qword)
}

#[test]
fn rol_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 4, 91], OperandSize::Qword)
}

#[test]
fn rol_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 193], OperandSize::Word)
}

#[test]
fn rol_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(DI, 49, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 69, 49], OperandSize::Word)
}

#[test]
fn rol_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 196], OperandSize::Dword)
}

#[test]
fn rol_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 4, 147], OperandSize::Dword)
}

#[test]
fn rol_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 197], OperandSize::Qword)
}

#[test]
fn rol_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 4, 209], OperandSize::Qword)
}

#[test]
fn rol_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 196], OperandSize::Word)
}

#[test]
fn rol_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 23020, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 131, 236, 89], OperandSize::Word)
}

#[test]
fn rol_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 194], OperandSize::Dword)
}

#[test]
fn rol_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 1], OperandSize::Dword)
}

#[test]
fn rol_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 198], OperandSize::Qword)
}

#[test]
fn rol_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 4, 131], OperandSize::Qword)
}

#[test]
fn rol_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RSP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 196], OperandSize::Qword)
}

#[test]
fn rol_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(RCX, Two, 629226547, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 4, 77, 51, 60, 129, 37], OperandSize::Qword)
}

