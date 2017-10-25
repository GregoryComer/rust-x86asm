use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rol_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 195, 116], OperandSize::Word)
}

#[test]
fn rol_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(BP, 97, Some(OperandSize::Byte), None)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 70, 97, 50], OperandSize::Word)
}

#[test]
fn rol_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 194, 17], OperandSize::Dword)
}

#[test]
fn rol_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 4, 247, 122], OperandSize::Dword)
}

#[test]
fn rol_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 193, 0], OperandSize::Qword)
}

#[test]
fn rol_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 170909273, Some(OperandSize::Byte), None)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 132, 203, 89, 222, 47, 10, 87], OperandSize::Qword)
}

#[test]
fn rol_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 194, 20], OperandSize::Qword)
}

#[test]
fn rol_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 2, 20], OperandSize::Qword)
}

#[test]
fn rol_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BP)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 197, 90], OperandSize::Word)
}

#[test]
fn rol_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 1, 32], OperandSize::Word)
}

#[test]
fn rol_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(SI)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 198, 16], OperandSize::Dword)
}

#[test]
fn rol_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1123405815, Some(OperandSize::Word), None)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 132, 87, 247, 207, 245, 66, 96], OperandSize::Dword)
}

#[test]
fn rol_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DI)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 199, 25], OperandSize::Qword)
}

#[test]
fn rol_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 4, 150, 127], OperandSize::Qword)
}

#[test]
fn rol_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDX)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 194, 126], OperandSize::Word)
}

#[test]
fn rol_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 5, 92], OperandSize::Word)
}

#[test]
fn rol_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 196, 7], OperandSize::Dword)
}

#[test]
fn rol_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(ESI, 796686640, Some(OperandSize::Dword), None)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 134, 48, 121, 124, 47, 62], OperandSize::Dword)
}

#[test]
fn rol_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 199, 37], OperandSize::Qword)
}

#[test]
fn rol_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RCX, 175064506, Some(OperandSize::Dword), None)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 129, 186, 69, 111, 10, 33], OperandSize::Qword)
}

#[test]
fn rol_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RCX)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 193, 126], OperandSize::Qword)
}

#[test]
fn rol_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RBX, 1397733216, Some(OperandSize::Qword), None)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 131, 96, 183, 79, 83, 67], OperandSize::Qword)
}

#[test]
fn rol_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 194], OperandSize::Word)
}

#[test]
fn rol_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 200, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 131, 200, 0], OperandSize::Word)
}

#[test]
fn rol_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 195], OperandSize::Dword)
}

#[test]
fn rol_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1478807271, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 132, 254, 231, 206, 36, 88], OperandSize::Dword)
}

#[test]
fn rol_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 195], OperandSize::Qword)
}

#[test]
fn rol_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(RCX, Four, 2121374850, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 4, 141, 130, 156, 113, 126], OperandSize::Qword)
}

#[test]
fn rol_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 193], OperandSize::Qword)
}

#[test]
fn rol_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RDX, 781005246, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 130, 190, 49, 141, 46], OperandSize::Qword)
}

#[test]
fn rol_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 196], OperandSize::Word)
}

#[test]
fn rol_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 114, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 66, 114], OperandSize::Word)
}

#[test]
fn rol_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 194], OperandSize::Dword)
}

#[test]
fn rol_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 0], OperandSize::Dword)
}

#[test]
fn rol_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 196], OperandSize::Qword)
}

#[test]
fn rol_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 0], OperandSize::Qword)
}

#[test]
fn rol_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 196], OperandSize::Word)
}

#[test]
fn rol_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(DI, 160, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 133, 160, 0], OperandSize::Word)
}

#[test]
fn rol_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 198], OperandSize::Dword)
}

#[test]
fn rol_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(ESI, 1084728231, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 134, 167, 163, 167, 64], OperandSize::Dword)
}

#[test]
fn rol_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 193], OperandSize::Qword)
}

#[test]
fn rol_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 4, 194], OperandSize::Qword)
}

#[test]
fn rol_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 195], OperandSize::Qword)
}

#[test]
fn rol_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RDI, 762924312, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 135, 24, 77, 121, 45], OperandSize::Qword)
}

#[test]
fn rol_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Word)
}

#[test]
fn rol_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(BX, 202, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 135, 202, 0], OperandSize::Word)
}

#[test]
fn rol_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 193], OperandSize::Dword)
}

#[test]
fn rol_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 921645005, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 132, 179, 205, 47, 239, 54], OperandSize::Dword)
}

#[test]
fn rol_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 193], OperandSize::Qword)
}

#[test]
fn rol_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 4, 86], OperandSize::Qword)
}

#[test]
fn rol_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Qword)
}

#[test]
fn rol_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 4, 206], OperandSize::Qword)
}

#[test]
fn rol_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 193], OperandSize::Word)
}

#[test]
fn rol_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 2], OperandSize::Word)
}

#[test]
fn rol_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 196], OperandSize::Dword)
}

#[test]
fn rol_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 3], OperandSize::Dword)
}

#[test]
fn rol_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 197], OperandSize::Qword)
}

#[test]
fn rol_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 4, 152], OperandSize::Qword)
}

#[test]
fn rol_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 194], OperandSize::Word)
}

#[test]
fn rol_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(DI, 31813, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 133, 69, 124], OperandSize::Word)
}

#[test]
fn rol_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 199], OperandSize::Dword)
}

#[test]
fn rol_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(EDI, Four, 372603843, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 4, 189, 195, 123, 53, 22], OperandSize::Dword)
}

#[test]
fn rol_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 196], OperandSize::Qword)
}

#[test]
fn rol_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RSI, 1233405649, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 134, 209, 70, 132, 73], OperandSize::Qword)
}

#[test]
fn rol_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RCX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 193], OperandSize::Qword)
}

#[test]
fn rol_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 131749824, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 132, 179, 192, 87, 218, 7], OperandSize::Qword)
}

