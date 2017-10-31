use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sar_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 251, 68], OperandSize::Word)
}

#[test]
fn sar_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(SI, 107, Some(OperandSize::Byte), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 124, 107, 7], OperandSize::Word)
}

#[test]
fn sar_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 251, 19], OperandSize::Dword)
}

#[test]
fn sar_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(EAX, 1064519809, Some(OperandSize::Byte), None)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 184, 129, 72, 115, 63, 50], OperandSize::Dword)
}

#[test]
fn sar_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 250, 26], OperandSize::Qword)
}

#[test]
fn sar_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RDI, 212091093, Some(OperandSize::Byte), None)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 191, 213, 64, 164, 12, 85], OperandSize::Qword)
}

#[test]
fn sar_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 251, 83], OperandSize::Qword)
}

#[test]
fn sar_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RBX, 685230481, Some(OperandSize::Byte), None)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 187, 145, 201, 215, 40, 45], OperandSize::Qword)
}

#[test]
fn sar_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CX)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 249, 100], OperandSize::Word)
}

#[test]
fn sar_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 60, 9], OperandSize::Word)
}

#[test]
fn sar_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BX)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 251, 104], OperandSize::Dword)
}

#[test]
fn sar_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 983290581, Some(OperandSize::Word), None)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 60, 253, 213, 210, 155, 58, 23], OperandSize::Dword)
}

#[test]
fn sar_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DI)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 255, 78], OperandSize::Qword)
}

#[test]
fn sar_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 609668571, Some(OperandSize::Word), None)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 60, 221, 219, 205, 86, 36, 31], OperandSize::Qword)
}

#[test]
fn sar_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 255, 61], OperandSize::Word)
}

#[test]
fn sar_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Memory(30501, Some(OperandSize::Dword), None)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 62, 37, 119, 12], OperandSize::Word)
}

#[test]
fn sar_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 252, 110], OperandSize::Dword)
}

#[test]
fn sar_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 60, 246, 35], OperandSize::Dword)
}

#[test]
fn sar_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 251, 16], OperandSize::Qword)
}

#[test]
fn sar_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RSI, Two, 1563006146, Some(OperandSize::Dword), None)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 60, 117, 194, 148, 41, 93, 127], OperandSize::Qword)
}

#[test]
fn sar_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RDX)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 250, 17], OperandSize::Qword)
}

#[test]
fn sar_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RDX, Four, 525139646, Some(OperandSize::Qword), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 60, 149, 190, 254, 76, 31, 113], OperandSize::Qword)
}

#[test]
fn sar_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 251], OperandSize::Word)
}

#[test]
fn sar_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 10367, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 184, 127, 40], OperandSize::Word)
}

#[test]
fn sar_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 251], OperandSize::Dword)
}

#[test]
fn sar_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1701150065, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 188, 81, 113, 125, 101, 101], OperandSize::Dword)
}

#[test]
fn sar_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 250], OperandSize::Qword)
}

#[test]
fn sar_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 60, 255], OperandSize::Qword)
}

#[test]
fn sar_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 250], OperandSize::Qword)
}

#[test]
fn sar_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1417260875, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 60, 69, 75, 175, 121, 84], OperandSize::Qword)
}

#[test]
fn sar_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 252], OperandSize::Word)
}

#[test]
fn sar_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 247, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 184, 247, 0], OperandSize::Word)
}

#[test]
fn sar_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 255], OperandSize::Dword)
}

#[test]
fn sar_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 60, 217], OperandSize::Dword)
}

#[test]
fn sar_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 251], OperandSize::Qword)
}

#[test]
fn sar_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1685649796, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 188, 208, 132, 249, 120, 100], OperandSize::Qword)
}

#[test]
fn sar_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 251], OperandSize::Word)
}

#[test]
fn sar_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(BP, 98, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 126, 98], OperandSize::Word)
}

#[test]
fn sar_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 251], OperandSize::Dword)
}

#[test]
fn sar_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 60, 80], OperandSize::Dword)
}

#[test]
fn sar_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 255], OperandSize::Qword)
}

#[test]
fn sar_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 60, 71], OperandSize::Qword)
}

#[test]
fn sar_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 253], OperandSize::Qword)
}

#[test]
fn sar_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 56], OperandSize::Qword)
}

#[test]
fn sar_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 250], OperandSize::Word)
}

#[test]
fn sar_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 57], OperandSize::Word)
}

#[test]
fn sar_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 251], OperandSize::Dword)
}

#[test]
fn sar_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 56], OperandSize::Dword)
}

#[test]
fn sar_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 249], OperandSize::Qword)
}

#[test]
fn sar_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1882726488, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 60, 125, 88, 32, 56, 112], OperandSize::Qword)
}

#[test]
fn sar_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 250], OperandSize::Qword)
}

#[test]
fn sar_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RBX, 1968859916, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 187, 12, 107, 90, 117], OperandSize::Qword)
}

#[test]
fn sar_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 250], OperandSize::Word)
}

#[test]
fn sar_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 60], OperandSize::Word)
}

#[test]
fn sar_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 249], OperandSize::Dword)
}

#[test]
fn sar_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1657146999, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 60, 125, 119, 14, 198, 98], OperandSize::Dword)
}

#[test]
fn sar_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 254], OperandSize::Qword)
}

#[test]
fn sar_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 547082407, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 188, 210, 167, 208, 155, 32], OperandSize::Qword)
}

#[test]
fn sar_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 251], OperandSize::Word)
}

#[test]
fn sar_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 61], OperandSize::Word)
}

#[test]
fn sar_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 249], OperandSize::Dword)
}

#[test]
fn sar_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(EDI, 856141092, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 191, 36, 173, 7, 51], OperandSize::Dword)
}

#[test]
fn sar_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 253], OperandSize::Qword)
}

#[test]
fn sar_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 513741433, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 188, 190, 121, 18, 159, 30], OperandSize::Qword)
}

#[test]
fn sar_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 255], OperandSize::Qword)
}

#[test]
fn sar_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RBX, Four, 327191612, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 60, 157, 60, 140, 128, 19], OperandSize::Qword)
}

