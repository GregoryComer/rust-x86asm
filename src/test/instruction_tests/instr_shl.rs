use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 226, 86], OperandSize::Word)
}

#[test]
fn shl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(BP, 25262, Some(OperandSize::Byte), None)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 166, 174, 98, 39], OperandSize::Word)
}

#[test]
fn shl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 226, 114], OperandSize::Dword)
}

#[test]
fn shl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1162802184, Some(OperandSize::Byte), None)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 164, 187, 8, 244, 78, 69, 38], OperandSize::Dword)
}

#[test]
fn shl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 28], OperandSize::Qword)
}

#[test]
fn shl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 363085646, Some(OperandSize::Byte), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 164, 146, 78, 63, 164, 21, 82], OperandSize::Qword)
}

#[test]
fn shl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 96], OperandSize::Qword)
}

#[test]
fn shl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 2130673815, Some(OperandSize::Byte), None)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 164, 176, 151, 128, 255, 126, 56], OperandSize::Qword)
}

#[test]
fn shl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BX)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 227, 90], OperandSize::Word)
}

#[test]
fn shl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(DI, 21495, Some(OperandSize::Word), None)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 165, 247, 83, 47], OperandSize::Word)
}

#[test]
fn shl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DI)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 231, 38], OperandSize::Dword)
}

#[test]
fn shl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(ECX, 1369833350, Some(OperandSize::Word), None)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 161, 134, 255, 165, 81, 52], OperandSize::Dword)
}

#[test]
fn shl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BP)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 229, 88], OperandSize::Qword)
}

#[test]
fn shl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 36, 80, 114], OperandSize::Qword)
}

#[test]
fn shl_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 231, 69], OperandSize::Word)
}

#[test]
fn shl_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 39, 7], OperandSize::Word)
}

#[test]
fn shl_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 225, 24], OperandSize::Dword)
}

#[test]
fn shl_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 39, 36], OperandSize::Dword)
}

#[test]
fn shl_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 231, 114], OperandSize::Qword)
}

#[test]
fn shl_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1522339807, Some(OperandSize::Dword), None)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 36, 181, 223, 15, 189, 90, 45], OperandSize::Qword)
}

#[test]
fn shl_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(RBX)), operand2: Some(Literal8(15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 227, 15], OperandSize::Qword)
}

#[test]
fn shl_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 346245900, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 164, 121, 12, 75, 163, 20], OperandSize::Qword)
}

#[test]
fn shl_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Word)
}

#[test]
fn shl_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 32], OperandSize::Word)
}

#[test]
fn shl_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Dword)
}

#[test]
fn shl_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 36, 65], OperandSize::Dword)
}

#[test]
fn shl_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Qword)
}

#[test]
fn shl_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 33], OperandSize::Qword)
}

#[test]
fn shl_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Qword)
}

#[test]
fn shl_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(RAX, 191228810, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 160, 138, 235, 101, 11], OperandSize::Qword)
}

#[test]
fn shl_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 231], OperandSize::Word)
}

#[test]
fn shl_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 115, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 97, 115], OperandSize::Word)
}

#[test]
fn shl_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 226], OperandSize::Dword)
}

#[test]
fn shl_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 39], OperandSize::Dword)
}

#[test]
fn shl_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 226], OperandSize::Qword)
}

#[test]
fn shl_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(RCX, 563382971, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 161, 187, 138, 148, 33], OperandSize::Qword)
}

#[test]
fn shl_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 228], OperandSize::Word)
}

#[test]
fn shl_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 169, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 160, 169, 0], OperandSize::Word)
}

#[test]
fn shl_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 229], OperandSize::Dword)
}

#[test]
fn shl_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 32], OperandSize::Dword)
}

#[test]
fn shl_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 228], OperandSize::Qword)
}

#[test]
fn shl_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1976100440, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 164, 254, 88, 230, 200, 117], OperandSize::Qword)
}

#[test]
fn shl_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(RSP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 228], OperandSize::Qword)
}

#[test]
fn shl_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 32], OperandSize::Qword)
}

#[test]
fn shl_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Word)
}

#[test]
fn shl_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(SI, 217, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 164, 217, 0], OperandSize::Word)
}

#[test]
fn shl_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Dword)
}

#[test]
fn shl_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36, 86], OperandSize::Dword)
}

#[test]
fn shl_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Qword)
}

#[test]
fn shl_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 34], OperandSize::Qword)
}

#[test]
fn shl_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Qword)
}

#[test]
fn shl_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36, 215], OperandSize::Qword)
}

#[test]
fn shl_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 227], OperandSize::Word)
}

#[test]
fn shl_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 39], OperandSize::Word)
}

#[test]
fn shl_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 230], OperandSize::Dword)
}

#[test]
fn shl_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(EDI, Four, 306119786, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 36, 189, 106, 4, 63, 18], OperandSize::Dword)
}

#[test]
fn shl_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 227], OperandSize::Qword)
}

#[test]
fn shl_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 36, 146], OperandSize::Qword)
}

#[test]
fn shl_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 230], OperandSize::Word)
}

#[test]
fn shl_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 33], OperandSize::Word)
}

#[test]
fn shl_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 229], OperandSize::Dword)
}

#[test]
fn shl_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(EDX, 160057105, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 162, 17, 71, 138, 9], OperandSize::Dword)
}

#[test]
fn shl_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 228], OperandSize::Qword)
}

#[test]
fn shl_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 968716239, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 164, 81, 207, 111, 189, 57], OperandSize::Qword)
}

#[test]
fn shl_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(RSI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 230], OperandSize::Qword)
}

#[test]
fn shl_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 36, 128], OperandSize::Qword)
}

