use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn add_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 209], OperandSize::Word)
}

#[test]
fn add_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 25785, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 139, 185, 100], OperandSize::Word)
}

#[test]
fn add_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 218], OperandSize::Dword)
}

#[test]
fn add_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 228662798, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 140, 216, 14, 30, 161, 13], OperandSize::Dword)
}

#[test]
fn add_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 202], OperandSize::Qword)
}

#[test]
fn add_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 12, 216], OperandSize::Qword)
}

#[test]
fn add_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 218], OperandSize::Qword)
}

#[test]
fn add_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RBX, 1007749033, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 147, 169, 7, 17, 60], OperandSize::Qword)
}

#[test]
fn add_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 234], OperandSize::Word)
}

#[test]
fn add_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 28372, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 161, 212, 110], OperandSize::Word)
}

#[test]
fn add_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 233], OperandSize::Dword)
}

#[test]
fn add_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 946999234, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 188, 143, 194, 15, 114, 56], OperandSize::Dword)
}

#[test]
fn add_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 206], OperandSize::Qword)
}

#[test]
fn add_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 41], OperandSize::Qword)
}

#[test]
fn add_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 206], OperandSize::Word)
}

#[test]
fn add_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(SI, 20533, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 172, 53, 80], OperandSize::Word)
}

#[test]
fn add_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 236], OperandSize::Dword)
}

#[test]
fn add_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(EDI, 1170733206, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 175, 150, 248, 199, 69], OperandSize::Dword)
}

#[test]
fn add_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 226], OperandSize::Qword)
}

#[test]
fn add_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 12, 71], OperandSize::Qword)
}

#[test]
fn add_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 241], OperandSize::Qword)
}

#[test]
fn add_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RBX, 1435852645, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 147, 101, 95, 149, 85], OperandSize::Qword)
}

#[test]
fn add_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 210], OperandSize::Word)
}

#[test]
fn add_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(SI, 187, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 140, 187, 0], OperandSize::Word)
}

#[test]
fn add_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 211], OperandSize::Dword)
}

#[test]
fn add_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(EDX, 773288928, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 146, 224, 115, 23, 46], OperandSize::Dword)
}

#[test]
fn add_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 218], OperandSize::Qword)
}

#[test]
fn add_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 2141383771, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 28, 149, 91, 236, 162, 127], OperandSize::Qword)
}

#[test]
fn add_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 202], OperandSize::Qword)
}

#[test]
fn add_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 12, 185], OperandSize::Qword)
}

#[test]
fn add_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 239], OperandSize::Word)
}

#[test]
fn add_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Memory(15937, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 62, 65, 62], OperandSize::Word)
}

#[test]
fn add_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 234], OperandSize::Dword)
}

#[test]
fn add_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(EDI, 765658762, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 143, 138, 6, 163, 45], OperandSize::Dword)
}

#[test]
fn add_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 238], OperandSize::Qword)
}

#[test]
fn add_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(RAX, 964917925, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 160, 165, 122, 131, 57], OperandSize::Qword)
}

#[test]
fn add_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 236], OperandSize::Word)
}

#[test]
fn add_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 17130, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 137, 234, 66], OperandSize::Word)
}

#[test]
fn add_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 242], OperandSize::Dword)
}

#[test]
fn add_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EAX, 1908846617, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 152, 25, 176, 198, 113], OperandSize::Dword)
}

#[test]
fn add_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 215], OperandSize::Qword)
}

#[test]
fn add_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1699602735, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 140, 75, 47, 225, 77, 101], OperandSize::Qword)
}

#[test]
fn add_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 226], OperandSize::Qword)
}

#[test]
fn add_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RDI)), operand2: Some(IndirectDisplaced(RBX, 738710381, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 3, 187, 109, 211, 7, 44], OperandSize::Qword)
}

#[test]
fn add_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 30], OperandSize::Word)
}

#[test]
fn add_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 73], OperandSize::Dword)
}

#[test]
fn add_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 59], OperandSize::Qword)
}

#[test]
fn add_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(15105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 1, 59], OperandSize::Word)
}

#[test]
fn add_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(25697)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 97, 100], OperandSize::Dword)
}

#[test]
fn add_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(24979)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 147, 97], OperandSize::Qword)
}

#[test]
fn add_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(899435730)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 210, 76, 156, 53], OperandSize::Word)
}

#[test]
fn add_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1397829597)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 221, 47, 81, 83], OperandSize::Dword)
}

#[test]
fn add_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1415510115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 99, 248, 94, 84], OperandSize::Qword)
}

#[test]
fn add_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1619530978)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 5, 226, 20, 136, 96], OperandSize::Qword)
}

#[test]
fn add_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 193, 16], OperandSize::Word)
}

#[test]
fn add_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 2, 6], OperandSize::Word)
}

#[test]
fn add_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 194, 48], OperandSize::Dword)
}

#[test]
fn add_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1701804862, Some(OperandSize::Byte), None)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 132, 146, 62, 123, 111, 101, 70], OperandSize::Dword)
}

#[test]
fn add_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 195, 48], OperandSize::Qword)
}

#[test]
fn add_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1093762416, Some(OperandSize::Byte), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 132, 80, 112, 125, 49, 65, 71], OperandSize::Qword)
}

#[test]
fn add_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 195, 41], OperandSize::Qword)
}

#[test]
fn add_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 1842947626, Some(OperandSize::Byte), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 132, 158, 42, 38, 217, 109, 104], OperandSize::Qword)
}

#[test]
fn add_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Literal16(3086)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 199, 14, 12], OperandSize::Word)
}

#[test]
fn add_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(DI, 97, Some(OperandSize::Word), None)), operand2: Some(Literal16(31881)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 69, 97, 137, 124], OperandSize::Word)
}

#[test]
fn add_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(Literal16(7071)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 196, 159, 27], OperandSize::Dword)
}

#[test]
fn add_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal16(16083)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 4, 128, 211, 62], OperandSize::Dword)
}

#[test]
fn add_67() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Literal16(1885)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 194, 93, 7], OperandSize::Qword)
}

#[test]
fn add_68() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RSI, 1874024360, Some(OperandSize::Word), None)), operand2: Some(Literal16(2338)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 134, 168, 87, 179, 111, 34, 9], OperandSize::Qword)
}

#[test]
fn add_69() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(Literal32(78966957)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 195, 173, 240, 180, 4], OperandSize::Word)
}

#[test]
fn add_70() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(BP, 63, Some(OperandSize::Dword), None)), operand2: Some(Literal32(326947646)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 70, 63, 62, 211, 124, 19], OperandSize::Word)
}

#[test]
fn add_71() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESP)), operand2: Some(Literal32(84861303)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 196, 119, 225, 14, 5], OperandSize::Dword)
}

#[test]
fn add_72() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1301020447, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1110140377)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 132, 127, 31, 255, 139, 77, 217, 101, 43, 66], OperandSize::Dword)
}

#[test]
fn add_73() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(Literal32(770332601)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 195, 185, 87, 234, 45], OperandSize::Qword)
}

#[test]
fn add_74() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1204482833)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 6, 17, 243, 202, 71], OperandSize::Qword)
}

#[test]
fn add_75() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RSP)), operand2: Some(Literal32(232565543)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 196, 39, 171, 220, 13], OperandSize::Qword)
}

#[test]
fn add_76() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 377556409, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1844423884)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 132, 87, 185, 13, 129, 22, 204, 172, 239, 109], OperandSize::Qword)
}

#[test]
fn add_77() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BX)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 195, 105], OperandSize::Word)
}

#[test]
fn add_78() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 142, Some(OperandSize::Word), None)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 129, 142, 0, 67], OperandSize::Word)
}

#[test]
fn add_79() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BX)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 195, 84], OperandSize::Dword)
}

#[test]
fn add_80() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 1997184152, Some(OperandSize::Word), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 132, 74, 152, 156, 10, 119, 113], OperandSize::Dword)
}

#[test]
fn add_81() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BX)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 195, 7], OperandSize::Qword)
}

#[test]
fn add_82() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1098956594, Some(OperandSize::Word), None)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 132, 138, 50, 191, 128, 65, 29], OperandSize::Qword)
}

#[test]
fn add_83() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDX)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 194, 21], OperandSize::Word)
}

#[test]
fn add_84() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(SI, 230, Some(OperandSize::Dword), None)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 132, 230, 0, 30], OperandSize::Word)
}

#[test]
fn add_85() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 197, 25], OperandSize::Dword)
}

#[test]
fn add_86() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 742961707, Some(OperandSize::Dword), None)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 4, 213, 43, 178, 72, 44, 50], OperandSize::Dword)
}

#[test]
fn add_87() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Literal8(13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 198, 13], OperandSize::Qword)
}

#[test]
fn add_88() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RSI, Four, 912280656, Some(OperandSize::Dword), None)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 4, 181, 80, 76, 96, 54, 91], OperandSize::Qword)
}

#[test]
fn add_89() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RDX)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 194, 76], OperandSize::Qword)
}

#[test]
fn add_90() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1604870897, Some(OperandSize::Qword), None)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 132, 179, 241, 98, 168, 95, 78], OperandSize::Qword)
}

