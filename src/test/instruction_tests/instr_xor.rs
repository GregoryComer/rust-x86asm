use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 211], OperandSize::Word)
}

#[test]
fn xor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 29], OperandSize::Word)
}

#[test]
fn xor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 219], OperandSize::Dword)
}

#[test]
fn xor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 28, 79], OperandSize::Dword)
}

#[test]
fn xor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 219], OperandSize::Qword)
}

#[test]
fn xor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 8], OperandSize::Qword)
}

#[test]
fn xor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 203], OperandSize::Qword)
}

#[test]
fn xor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RAX, Four, 346115368, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 28, 133, 40, 77, 161, 20], OperandSize::Qword)
}

#[test]
fn xor_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 207], OperandSize::Word)
}

#[test]
fn xor_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 25], OperandSize::Word)
}

#[test]
fn xor_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 255], OperandSize::Dword)
}

#[test]
fn xor_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 33], OperandSize::Dword)
}

#[test]
fn xor_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 221], OperandSize::Qword)
}

#[test]
fn xor_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 60, 158], OperandSize::Qword)
}

#[test]
fn xor_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 230], OperandSize::Word)
}

#[test]
fn xor_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(BX, 236, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 175, 236, 0], OperandSize::Word)
}

#[test]
fn xor_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 225], OperandSize::Dword)
}

#[test]
fn xor_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(EDI, 93461447, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 167, 199, 27, 146, 5], OperandSize::Dword)
}

#[test]
fn xor_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 211], OperandSize::Qword)
}

#[test]
fn xor_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 25], OperandSize::Qword)
}

#[test]
fn xor_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 207], OperandSize::Qword)
}

#[test]
fn xor_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 12, 178], OperandSize::Qword)
}

#[test]
fn xor_23() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 211], OperandSize::Word)
}

#[test]
fn xor_24() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 9], OperandSize::Word)
}

#[test]
fn xor_25() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 219], OperandSize::Dword)
}

#[test]
fn xor_26() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 27], OperandSize::Dword)
}

#[test]
fn xor_27() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 202], OperandSize::Qword)
}

#[test]
fn xor_28() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RCX, 1669154285, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 137, 237, 69, 125, 99], OperandSize::Qword)
}

#[test]
fn xor_29() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 217], OperandSize::Qword)
}

#[test]
fn xor_30() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 10987856, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 156, 211, 80, 169, 167, 0], OperandSize::Qword)
}

#[test]
fn xor_31() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 214], OperandSize::Word)
}

#[test]
fn xor_32() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(BX, 15233, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 167, 129, 59], OperandSize::Word)
}

#[test]
fn xor_33() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 207], OperandSize::Dword)
}

#[test]
fn xor_34() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1712155783, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 36, 93, 135, 108, 13, 102], OperandSize::Dword)
}

#[test]
fn xor_35() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 206], OperandSize::Qword)
}

#[test]
fn xor_36() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 20, 143], OperandSize::Qword)
}

#[test]
fn xor_37() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 254], OperandSize::Word)
}

#[test]
fn xor_38() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 56], OperandSize::Word)
}

#[test]
fn xor_39() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 201], OperandSize::Dword)
}

#[test]
fn xor_40() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1508805628, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 20, 205, 252, 139, 238, 89], OperandSize::Dword)
}

#[test]
fn xor_41() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 205], OperandSize::Qword)
}

#[test]
fn xor_42() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 531296093, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 36, 149, 93, 239, 170, 31], OperandSize::Qword)
}

#[test]
fn xor_43() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 255], OperandSize::Qword)
}

#[test]
fn xor_44() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1667937052, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 51, 148, 130, 28, 179, 106, 99], OperandSize::Qword)
}

#[test]
fn xor_45() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 55], OperandSize::Word)
}

#[test]
fn xor_46() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 106], OperandSize::Dword)
}

#[test]
fn xor_47() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 43], OperandSize::Qword)
}

#[test]
fn xor_48() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(19139)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 195, 74], OperandSize::Word)
}

#[test]
fn xor_49() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(11329)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 65, 44], OperandSize::Dword)
}

#[test]
fn xor_50() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(1067)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 43, 4], OperandSize::Qword)
}

#[test]
fn xor_51() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1059323205)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 69, 253, 35, 63], OperandSize::Word)
}

#[test]
fn xor_52() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1147809785)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 249, 47, 106, 68], OperandSize::Dword)
}

#[test]
fn xor_53() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1131202224)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 176, 198, 108, 67], OperandSize::Qword)
}

#[test]
fn xor_54() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RAX)), operand2: Some(Literal32(767259715)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 53, 67, 116, 187, 45], OperandSize::Qword)
}

#[test]
fn xor_55() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 243, 41], OperandSize::Word)
}

#[test]
fn xor_56() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 23655, Some(OperandSize::Byte), None)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 176, 103, 92, 98], OperandSize::Word)
}

#[test]
fn xor_57() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 243, 115], OperandSize::Dword)
}

#[test]
fn xor_58() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1044136659, Some(OperandSize::Byte), None)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 180, 241, 211, 66, 60, 62, 85], OperandSize::Dword)
}

#[test]
fn xor_59() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 242, 89], OperandSize::Qword)
}

#[test]
fn xor_60() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RCX, Two, 35507939, Some(OperandSize::Byte), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 52, 77, 227, 206, 29, 2, 110], OperandSize::Qword)
}

#[test]
fn xor_61() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 241, 38], OperandSize::Qword)
}

#[test]
fn xor_62() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RAX, 537158220, Some(OperandSize::Byte), None)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 176, 76, 98, 4, 32, 85], OperandSize::Qword)
}

#[test]
fn xor_63() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Literal16(1346)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 247, 66, 5], OperandSize::Word)
}

#[test]
fn xor_64() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 20898, Some(OperandSize::Word), None)), operand2: Some(Literal16(18118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 179, 162, 81, 198, 70], OperandSize::Word)
}

#[test]
fn xor_65() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Literal16(16956)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 247, 60, 66], OperandSize::Dword)
}

#[test]
fn xor_66() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal16(4664)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 52, 94, 56, 18], OperandSize::Dword)
}

#[test]
fn xor_67() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Literal16(386)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 247, 130, 1], OperandSize::Qword)
}

#[test]
fn xor_68() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal16(23762)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 52, 199, 210, 92], OperandSize::Qword)
}

#[test]
fn xor_69() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDI)), operand2: Some(Literal32(809515991)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 247, 215, 59, 64, 48], OperandSize::Word)
}

#[test]
fn xor_70() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1540427709)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 55, 189, 15, 209, 91], OperandSize::Word)
}

#[test]
fn xor_71() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Literal32(2131461513)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 245, 137, 133, 11, 127], OperandSize::Dword)
}

#[test]
fn xor_72() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1323182867, Some(OperandSize::Dword), None)), operand2: Some(Literal32(204754506)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 52, 93, 19, 43, 222, 78, 74, 78, 52, 12], OperandSize::Dword)
}

#[test]
fn xor_73() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Literal32(375539946)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 245, 234, 72, 98, 22], OperandSize::Qword)
}

#[test]
fn xor_74() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1178778203)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 52, 200, 91, 186, 66, 70], OperandSize::Qword)
}

#[test]
fn xor_75() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDI)), operand2: Some(Literal32(189111697)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 247, 145, 157, 69, 11], OperandSize::Qword)
}

#[test]
fn xor_76() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Literal32(279483870)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 51, 222, 149, 168, 16], OperandSize::Qword)
}

#[test]
fn xor_77() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 244, 71], OperandSize::Word)
}

#[test]
fn xor_78() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 49, 110], OperandSize::Word)
}

#[test]
fn xor_79() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 246, 107], OperandSize::Dword)
}

#[test]
fn xor_80() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 2020708721, Some(OperandSize::Word), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 52, 221, 113, 145, 113, 120, 105], OperandSize::Dword)
}

#[test]
fn xor_81() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 247, 101], OperandSize::Qword)
}

#[test]
fn xor_82() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RCX, Four, 1345858935, Some(OperandSize::Word), None)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 52, 141, 119, 45, 56, 80, 29], OperandSize::Qword)
}

#[test]
fn xor_83() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 245, 60], OperandSize::Word)
}

#[test]
fn xor_84() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(BX, 179, Some(OperandSize::Dword), None)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 183, 179, 0, 85], OperandSize::Word)
}

#[test]
fn xor_85() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 243, 41], OperandSize::Dword)
}

#[test]
fn xor_86() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(ESI, 232976397, Some(OperandSize::Dword), None)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 182, 13, 240, 226, 13, 17], OperandSize::Dword)
}

#[test]
fn xor_87() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(124)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 244, 124], OperandSize::Qword)
}

#[test]
fn xor_88() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RDX, Four, 113113451, Some(OperandSize::Dword), None)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 52, 149, 107, 249, 189, 6, 38], OperandSize::Qword)
}

#[test]
fn xor_89() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDX)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 242, 10], OperandSize::Qword)
}

#[test]
fn xor_90() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1543346355, Some(OperandSize::Qword), None)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 180, 248, 179, 152, 253, 91, 3], OperandSize::Qword)
}

