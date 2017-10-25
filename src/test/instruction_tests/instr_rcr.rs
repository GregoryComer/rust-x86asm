use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 219, 106], OperandSize::Word)
}

#[test]
fn rcr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 27, 43], OperandSize::Word)
}

#[test]
fn rcr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 219, 36], OperandSize::Dword)
}

#[test]
fn rcr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(EAX, Four, 432698872, Some(OperandSize::Byte), None)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 28, 133, 248, 117, 202, 25, 106], OperandSize::Dword)
}

#[test]
fn rcr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 217, 101], OperandSize::Qword)
}

#[test]
fn rcr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(102)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 30, 102], OperandSize::Qword)
}

#[test]
fn rcr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 219, 116], OperandSize::Qword)
}

#[test]
fn rcr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 118586888, Some(OperandSize::Byte), None)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 156, 95, 8, 126, 17, 7, 16], OperandSize::Qword)
}

#[test]
fn rcr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DX)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 218, 5], OperandSize::Word)
}

#[test]
fn rcr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(SI, 4656, Some(OperandSize::Word), None)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 156, 48, 18, 98], OperandSize::Word)
}

#[test]
fn rcr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BP)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 221, 48], OperandSize::Dword)
}

#[test]
fn rcr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(EDI, 647600672, Some(OperandSize::Word), None)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 159, 32, 154, 153, 38, 58], OperandSize::Dword)
}

#[test]
fn rcr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DX)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 218, 41], OperandSize::Qword)
}

#[test]
fn rcr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 25, 7], OperandSize::Qword)
}

#[test]
fn rcr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 221, 126], OperandSize::Word)
}

#[test]
fn rcr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 26, 87], OperandSize::Word)
}

#[test]
fn rcr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 219, 33], OperandSize::Dword)
}

#[test]
fn rcr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 28, 208, 10], OperandSize::Dword)
}

#[test]
fn rcr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 219, 51], OperandSize::Qword)
}

#[test]
fn rcr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1337213717, Some(OperandSize::Dword), None)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 28, 125, 21, 67, 180, 79, 116], OperandSize::Qword)
}

#[test]
fn rcr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RBP)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 221, 96], OperandSize::Qword)
}

#[test]
fn rcr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 24, 61], OperandSize::Qword)
}

#[test]
fn rcr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 219], OperandSize::Word)
}

#[test]
fn rcr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 28], OperandSize::Word)
}

#[test]
fn rcr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 218], OperandSize::Dword)
}

#[test]
fn rcr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1287902087, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 156, 72, 135, 211, 195, 76], OperandSize::Dword)
}

#[test]
fn rcr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 217], OperandSize::Qword)
}

#[test]
fn rcr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(RAX, 1455224675, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 152, 99, 247, 188, 86], OperandSize::Qword)
}

#[test]
fn rcr_29() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 219], OperandSize::Qword)
}

#[test]
fn rcr_30() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 205277617, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 28, 253, 177, 73, 60, 12], OperandSize::Qword)
}

#[test]
fn rcr_31() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 220], OperandSize::Word)
}

#[test]
fn rcr_32() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 27], OperandSize::Word)
}

#[test]
fn rcr_33() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 222], OperandSize::Dword)
}

#[test]
fn rcr_34() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 499450499, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 156, 120, 131, 2, 197, 29], OperandSize::Dword)
}

#[test]
fn rcr_35() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 217], OperandSize::Qword)
}

#[test]
fn rcr_36() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 28, 246], OperandSize::Qword)
}

#[test]
fn rcr_37() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 220], OperandSize::Word)
}

#[test]
fn rcr_38() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 27], OperandSize::Word)
}

#[test]
fn rcr_39() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 217], OperandSize::Dword)
}

#[test]
fn rcr_40() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 28, 155], OperandSize::Dword)
}

#[test]
fn rcr_41() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 217], OperandSize::Qword)
}

#[test]
fn rcr_42() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 28, 195], OperandSize::Qword)
}

#[test]
fn rcr_43() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 219], OperandSize::Qword)
}

#[test]
fn rcr_44() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectDisplaced(RDX, 1976380360, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 154, 200, 43, 205, 117], OperandSize::Qword)
}

#[test]
fn rcr_45() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 217], OperandSize::Word)
}

#[test]
fn rcr_46() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 28], OperandSize::Word)
}

#[test]
fn rcr_47() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 217], OperandSize::Dword)
}

#[test]
fn rcr_48() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1959626986, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 28, 253, 234, 136, 205, 116], OperandSize::Dword)
}

#[test]
fn rcr_49() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 217], OperandSize::Qword)
}

#[test]
fn rcr_50() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 28, 185], OperandSize::Qword)
}

#[test]
fn rcr_51() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 218], OperandSize::Qword)
}

#[test]
fn rcr_52() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 1397309035, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 156, 199, 107, 62, 73, 83], OperandSize::Qword)
}

#[test]
fn rcr_53() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 217], OperandSize::Word)
}

#[test]
fn rcr_54() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 50, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 89, 50], OperandSize::Word)
}

#[test]
fn rcr_55() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 219], OperandSize::Dword)
}

#[test]
fn rcr_56() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 27], OperandSize::Dword)
}

#[test]
fn rcr_57() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 221], OperandSize::Qword)
}

#[test]
fn rcr_58() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 415027194, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 156, 83, 250, 207, 188, 24], OperandSize::Qword)
}

#[test]
fn rcr_59() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 218], OperandSize::Word)
}

#[test]
fn rcr_60() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 14235, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 152, 155, 55], OperandSize::Word)
}

#[test]
fn rcr_61() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 220], OperandSize::Dword)
}

#[test]
fn rcr_62() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 31], OperandSize::Dword)
}

#[test]
fn rcr_63() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 221], OperandSize::Qword)
}

#[test]
fn rcr_64() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 28, 94], OperandSize::Qword)
}

#[test]
fn rcr_65() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Direct(RSI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 222], OperandSize::Qword)
}

#[test]
fn rcr_66() {
    run_test(&Instruction { mnemonic: Mnemonic::RCR, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 24], OperandSize::Qword)
}

