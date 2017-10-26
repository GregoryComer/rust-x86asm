use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sal_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 226, 65], OperandSize::Word)
}

#[test]
fn sal_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(BX, 1066, Some(OperandSize::Byte), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 167, 42, 4, 113], OperandSize::Word)
}

#[test]
fn sal_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(119)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 119], OperandSize::Dword)
}

#[test]
fn sal_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(EDX, 272028646, Some(OperandSize::Byte), None)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 162, 230, 211, 54, 16, 86], OperandSize::Dword)
}

#[test]
fn sal_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 225, 16], OperandSize::Qword)
}

#[test]
fn sal_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 192, 71], OperandSize::Qword)
}

#[test]
fn sal_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 225, 31], OperandSize::Qword)
}

#[test]
fn sal_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 71, 77], OperandSize::Qword)
}

#[test]
fn sal_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CX)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 225, 66], OperandSize::Word)
}

#[test]
fn sal_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Memory(17460, Some(OperandSize::Word), None)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 38, 52, 68, 21], OperandSize::Word)
}

#[test]
fn sal_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DX)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 226, 36], OperandSize::Dword)
}

#[test]
fn sal_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 38, 41], OperandSize::Dword)
}

#[test]
fn sal_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BX)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 227, 4], OperandSize::Qword)
}

#[test]
fn sal_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(RAX, 427344589, Some(OperandSize::Word), None)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 160, 205, 194, 120, 25, 53], OperandSize::Qword)
}

#[test]
fn sal_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 229, 89], OperandSize::Word)
}

#[test]
fn sal_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(BP, 156, Some(OperandSize::Dword), None)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 166, 156, 0, 19], OperandSize::Word)
}

#[test]
fn sal_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 228, 51], OperandSize::Dword)
}

#[test]
fn sal_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(ESI, 2143589170, Some(OperandSize::Dword), None)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 166, 50, 147, 196, 127, 43], OperandSize::Dword)
}

#[test]
fn sal_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 230, 112], OperandSize::Qword)
}

#[test]
fn sal_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 995182369, Some(OperandSize::Dword), None)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 164, 243, 33, 71, 81, 59, 125], OperandSize::Qword)
}

#[test]
fn sal_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RSI)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 230, 117], OperandSize::Qword)
}

#[test]
fn sal_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 33, 71], OperandSize::Qword)
}

#[test]
fn sal_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Word)
}

#[test]
fn sal_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 38, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 99, 38], OperandSize::Word)
}

#[test]
fn sal_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 226], OperandSize::Dword)
}

#[test]
fn sal_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(EDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 34], OperandSize::Dword)
}

#[test]
fn sal_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Qword)
}

#[test]
fn sal_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(RSI, 1302504084, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 166, 148, 162, 162, 77], OperandSize::Qword)
}

#[test]
fn sal_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Qword)
}

#[test]
fn sal_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(RSI, 227269234, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 166, 114, 218, 139, 13], OperandSize::Qword)
}

#[test]
fn sal_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 229], OperandSize::Word)
}

#[test]
fn sal_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 6464, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 160, 64, 25], OperandSize::Word)
}

#[test]
fn sal_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 231], OperandSize::Dword)
}

#[test]
fn sal_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 169638367, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 36, 197, 223, 121, 28, 10], OperandSize::Dword)
}

#[test]
fn sal_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 225], OperandSize::Qword)
}

#[test]
fn sal_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 33], OperandSize::Qword)
}

#[test]
fn sal_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 231], OperandSize::Word)
}

#[test]
fn sal_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(BP, 110, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 102, 110], OperandSize::Word)
}

#[test]
fn sal_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 226], OperandSize::Dword)
}

#[test]
fn sal_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 36, 137], OperandSize::Dword)
}

#[test]
fn sal_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 228], OperandSize::Qword)
}

#[test]
fn sal_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 164418943, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 164, 88, 127, 213, 204, 9], OperandSize::Qword)
}

#[test]
fn sal_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RCX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 225], OperandSize::Qword)
}

#[test]
fn sal_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 76884816, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 164, 74, 80, 43, 149, 4], OperandSize::Qword)
}

#[test]
fn sal_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Word)
}

#[test]
fn sal_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(DI, 244, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 165, 244, 0], OperandSize::Word)
}

#[test]
fn sal_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Dword)
}

#[test]
fn sal_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(EDI, 1410914968, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 167, 152, 218, 24, 84], OperandSize::Dword)
}

#[test]
fn sal_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Qword)
}

#[test]
fn sal_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(RSI, 697747654, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 166, 198, 200, 150, 41], OperandSize::Qword)
}

#[test]
fn sal_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 226], OperandSize::Qword)
}

#[test]
fn sal_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RAX, Two, 184421278, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36, 69, 158, 11, 254, 10], OperandSize::Qword)
}

#[test]
fn sal_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 226], OperandSize::Word)
}

#[test]
fn sal_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 35], OperandSize::Word)
}

#[test]
fn sal_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 226], OperandSize::Dword)
}

#[test]
fn sal_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 32], OperandSize::Dword)
}

#[test]
fn sal_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 227], OperandSize::Qword)
}

#[test]
fn sal_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 2111880807, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 164, 182, 103, 190, 224, 125], OperandSize::Qword)
}

#[test]
fn sal_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 231], OperandSize::Word)
}

#[test]
fn sal_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 8890, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 162, 186, 34], OperandSize::Word)
}

#[test]
fn sal_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 226], OperandSize::Dword)
}

#[test]
fn sal_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1688684985, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 36, 245, 185, 73, 167, 100], OperandSize::Dword)
}

#[test]
fn sal_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 226], OperandSize::Qword)
}

#[test]
fn sal_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 559823219, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 164, 128, 115, 57, 94, 33], OperandSize::Qword)
}

#[test]
fn sal_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 229], OperandSize::Qword)
}

#[test]
fn sal_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 32], OperandSize::Qword)
}

