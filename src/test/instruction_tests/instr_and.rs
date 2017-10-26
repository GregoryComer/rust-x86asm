use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn and_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 202], OperandSize::Word)
}

#[test]
fn and_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 20169, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 138, 201, 78], OperandSize::Word)
}

#[test]
fn and_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 201], OperandSize::Dword)
}

#[test]
fn and_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(ECX, Four, 850238662, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 12, 141, 198, 156, 173, 50], OperandSize::Dword)
}

#[test]
fn and_5() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 217], OperandSize::Qword)
}

#[test]
fn and_6() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RSI, 1623709298, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 142, 114, 214, 199, 96], OperandSize::Qword)
}

#[test]
fn and_7() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 211], OperandSize::Qword)
}

#[test]
fn and_8() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 20, 67], OperandSize::Qword)
}

#[test]
fn and_9() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 230], OperandSize::Word)
}

#[test]
fn and_10() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Memory(21099, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 46, 107, 82], OperandSize::Word)
}

#[test]
fn and_11() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 219], OperandSize::Dword)
}

#[test]
fn and_12() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 12, 143], OperandSize::Dword)
}

#[test]
fn and_13() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 227], OperandSize::Qword)
}

#[test]
fn and_14() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 39], OperandSize::Qword)
}

#[test]
fn and_15() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 233], OperandSize::Word)
}

#[test]
fn and_16() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 18649, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 168, 217, 72], OperandSize::Word)
}

#[test]
fn and_17() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 207], OperandSize::Dword)
}

#[test]
fn and_18() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 673865255, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 188, 214, 39, 94, 42, 40], OperandSize::Dword)
}

#[test]
fn and_19() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 226], OperandSize::Qword)
}

#[test]
fn and_20() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RAX, 1464384477, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 160, 221, 187, 72, 87], OperandSize::Qword)
}

#[test]
fn and_21() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 255], OperandSize::Qword)
}

#[test]
fn and_22() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 44, 134], OperandSize::Qword)
}

#[test]
fn and_23() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 219], OperandSize::Word)
}

#[test]
fn and_24() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(BX, 189, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 143, 189, 0], OperandSize::Word)
}

#[test]
fn and_25() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 211], OperandSize::Dword)
}

#[test]
fn and_26() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 15], OperandSize::Dword)
}

#[test]
fn and_27() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 202], OperandSize::Qword)
}

#[test]
fn and_28() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 753180575, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 28, 213, 159, 159, 228, 44], OperandSize::Qword)
}

#[test]
fn and_29() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[32, 209], OperandSize::Qword)
}

#[test]
fn and_30() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1103502385, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[34, 156, 75, 49, 28, 198, 65], OperandSize::Qword)
}

#[test]
fn and_31() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 255], OperandSize::Word)
}

#[test]
fn and_32() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 16537, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 147, 153, 64], OperandSize::Word)
}

#[test]
fn and_33() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 217], OperandSize::Dword)
}

#[test]
fn and_34() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 2046766208, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 180, 223, 128, 44, 255, 121], OperandSize::Dword)
}

#[test]
fn and_35() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 239], OperandSize::Qword)
}

#[test]
fn and_36() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1969697888, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 36, 77, 96, 52, 103, 117], OperandSize::Qword)
}

#[test]
fn and_37() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 33, 214], OperandSize::Word)
}

#[test]
fn and_38() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 35, 40], OperandSize::Word)
}

#[test]
fn and_39() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 241], OperandSize::Dword)
}

#[test]
fn and_40() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(EDI, 263701593, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 151, 89, 196, 183, 15], OperandSize::Dword)
}

#[test]
fn and_41() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[33, 202], OperandSize::Qword)
}

#[test]
fn and_42() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[35, 24], OperandSize::Qword)
}

#[test]
fn and_43() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 33, 227], OperandSize::Qword)
}

#[test]
fn and_44() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 35, 20, 184], OperandSize::Qword)
}

#[test]
fn and_45() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 15], OperandSize::Word)
}

#[test]
fn and_46() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 110], OperandSize::Dword)
}

#[test]
fn and_47() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AL)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[36, 127], OperandSize::Qword)
}

#[test]
fn and_48() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(31611)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 123, 123], OperandSize::Word)
}

#[test]
fn and_49() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(2597)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 37, 10], OperandSize::Dword)
}

#[test]
fn and_50() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(AX)), operand2: Some(Literal16(18565)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 133, 72], OperandSize::Qword)
}

#[test]
fn and_51() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(750623946)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 37, 202, 156, 189, 44], OperandSize::Word)
}

#[test]
fn and_52() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(676941657)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 89, 79, 89, 40], OperandSize::Dword)
}

#[test]
fn and_53() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1021434851)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[37, 227, 219, 225, 60], OperandSize::Qword)
}

#[test]
fn and_54() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RAX)), operand2: Some(Literal32(2140686680)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 37, 88, 73, 152, 127], OperandSize::Qword)
}

#[test]
fn and_55() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 227, 63], OperandSize::Word)
}

#[test]
fn and_56() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(BX, 26825, Some(OperandSize::Byte), None)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 167, 201, 104, 50], OperandSize::Word)
}

#[test]
fn and_57() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BL)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 227, 30], OperandSize::Dword)
}

#[test]
fn and_58() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 783562232, Some(OperandSize::Byte), None)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 36, 221, 248, 53, 180, 46, 33], OperandSize::Dword)
}

#[test]
fn and_59() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(CL)), operand2: Some(Literal8(8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 225, 8], OperandSize::Qword)
}

#[test]
fn and_60() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 38, 12], OperandSize::Qword)
}

#[test]
fn and_61() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DL)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 226, 10], OperandSize::Qword)
}

#[test]
fn and_62() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(RSI, Two, 2134016299, Some(OperandSize::Byte), None)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 36, 117, 43, 129, 50, 127, 9], OperandSize::Qword)
}

#[test]
fn and_63() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(DX)), operand2: Some(Literal16(8245)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 226, 53, 32], OperandSize::Word)
}

#[test]
fn and_64() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(BP, 15225, Some(OperandSize::Word), None)), operand2: Some(Literal16(30479)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 166, 121, 59, 15, 119], OperandSize::Word)
}

#[test]
fn and_65() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(Literal16(31625)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 230, 137, 123], OperandSize::Dword)
}

#[test]
fn and_66() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 299207069, Some(OperandSize::Word), None)), operand2: Some(Literal16(9932)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 164, 119, 157, 137, 213, 17, 204, 38], OperandSize::Dword)
}

#[test]
fn and_67() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SI)), operand2: Some(Literal16(28051)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 230, 147, 109], OperandSize::Qword)
}

#[test]
fn and_68() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 310753233, Some(OperandSize::Word), None)), operand2: Some(Literal16(14488)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 36, 253, 209, 183, 133, 18, 152, 56], OperandSize::Qword)
}

#[test]
fn and_69() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1354029536)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 229, 224, 217, 180, 80], OperandSize::Word)
}

#[test]
fn and_70() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 169, Some(OperandSize::Dword), None)), operand2: Some(Literal32(735644095)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 161, 169, 0, 191, 9, 217, 43], OperandSize::Word)
}

#[test]
fn and_71() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDI)), operand2: Some(Literal32(251341259)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 231, 203, 41, 251, 14], OperandSize::Dword)
}

#[test]
fn and_72() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1641462706, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1070797503)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 164, 182, 178, 187, 214, 97, 191, 18, 211, 63], OperandSize::Dword)
}

#[test]
fn and_73() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Literal32(451852075)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 227, 43, 183, 238, 26], OperandSize::Qword)
}

#[test]
fn and_74() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 498386026, Some(OperandSize::Dword), None)), operand2: Some(Literal32(990673427)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 164, 242, 106, 196, 180, 29, 19, 122, 12, 59], OperandSize::Qword)
}

#[test]
fn and_75() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RDI)), operand2: Some(Literal32(873249632)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 231, 96, 187, 12, 52], OperandSize::Qword)
}

#[test]
fn and_76() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Literal32(783943043)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 35, 131, 5, 186, 46], OperandSize::Qword)
}

#[test]
fn and_77() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SP)), operand2: Some(Literal8(13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 228, 13], OperandSize::Word)
}

#[test]
fn and_78() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 171, Some(OperandSize::Word), None)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 161, 171, 0, 101], OperandSize::Word)
}

#[test]
fn and_79() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 228, 1], OperandSize::Dword)
}

#[test]
fn and_80() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 568710400, Some(OperandSize::Word), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 164, 246, 0, 213, 229, 33, 113], OperandSize::Dword)
}

#[test]
fn and_81() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(BP)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 229, 81], OperandSize::Qword)
}

#[test]
fn and_82() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 788255175, Some(OperandSize::Word), None)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 164, 243, 199, 209, 251, 46, 10], OperandSize::Qword)
}

#[test]
fn and_83() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(ECX)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 225, 89], OperandSize::Word)
}

#[test]
fn and_84() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 33, 72], OperandSize::Word)
}

#[test]
fn and_85() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EBX)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 227, 15], OperandSize::Dword)
}

#[test]
fn and_86() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1357959949, Some(OperandSize::Dword), None)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 36, 245, 13, 211, 240, 80, 31], OperandSize::Dword)
}

#[test]
fn and_87() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(EDX)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 226, 126], OperandSize::Qword)
}

#[test]
fn and_88() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(IndirectDisplaced(RDI, 672023941, Some(OperandSize::Dword), None)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 167, 133, 69, 14, 40, 22], OperandSize::Qword)
}

#[test]
fn and_89() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Direct(RDX)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 226, 103], OperandSize::Qword)
}

#[test]
fn and_90() {
    run_test(&Instruction { mnemonic: Mnemonic::AND, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 32, 27], OperandSize::Qword)
}

