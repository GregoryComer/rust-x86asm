use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CL)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 233, 59], OperandSize::Word)
}

#[test]
fn shr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(BX, 106, Some(OperandSize::Byte), None)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 111, 106, 70], OperandSize::Word)
}

#[test]
fn shr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CL)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 233, 78], OperandSize::Dword)
}

#[test]
fn shr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(ECX, 1653651443, Some(OperandSize::Byte), None)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 169, 243, 183, 144, 98, 101], OperandSize::Dword)
}

#[test]
fn shr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 234, 58], OperandSize::Qword)
}

#[test]
fn shr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 41, 120], OperandSize::Qword)
}

#[test]
fn shr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 235, 10], OperandSize::Qword)
}

#[test]
fn shr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 44, 146, 26], OperandSize::Qword)
}

#[test]
fn shr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DI)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 239, 125], OperandSize::Word)
}

#[test]
fn shr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 42, 91], OperandSize::Word)
}

#[test]
fn shr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CX)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 233, 2], OperandSize::Dword)
}

#[test]
fn shr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 46, 96], OperandSize::Dword)
}

#[test]
fn shr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CX)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 233, 52], OperandSize::Qword)
}

#[test]
fn shr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1997550189, Some(OperandSize::Word), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 44, 221, 109, 50, 16, 119, 115], OperandSize::Qword)
}

#[test]
fn shr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 238, 77], OperandSize::Word)
}

#[test]
fn shr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 41, 98], OperandSize::Word)
}

#[test]
fn shr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 234, 107], OperandSize::Dword)
}

#[test]
fn shr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(ESI, 1124780003, Some(OperandSize::Dword), None)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 174, 227, 199, 10, 67, 69], OperandSize::Dword)
}

#[test]
fn shr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 235, 116], OperandSize::Qword)
}

#[test]
fn shr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1850940185, Some(OperandSize::Dword), None)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 44, 181, 25, 27, 83, 110, 19], OperandSize::Qword)
}

#[test]
fn shr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(RDX)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 234, 29], OperandSize::Qword)
}

#[test]
fn shr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 1008924579, Some(OperandSize::Qword), None)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 172, 218, 163, 247, 34, 60, 21], OperandSize::Qword)
}

#[test]
fn shr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 235], OperandSize::Word)
}

#[test]
fn shr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 25977, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 170, 121, 101], OperandSize::Word)
}

#[test]
fn shr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 234], OperandSize::Dword)
}

#[test]
fn shr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(ECX, 1967737736, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 169, 136, 75, 73, 117], OperandSize::Dword)
}

#[test]
fn shr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 234], OperandSize::Qword)
}

#[test]
fn shr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 42], OperandSize::Qword)
}

#[test]
fn shr_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 234], OperandSize::Qword)
}

#[test]
fn shr_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RDX, 1171300430, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 170, 78, 160, 208, 69], OperandSize::Qword)
}

#[test]
fn shr_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 233], OperandSize::Word)
}

#[test]
fn shr_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 44, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 107, 44], OperandSize::Word)
}

#[test]
fn shr_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 234], OperandSize::Dword)
}

#[test]
fn shr_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(EDX, Two, 544345947, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 44, 85, 91, 15, 114, 32], OperandSize::Dword)
}

#[test]
fn shr_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 238], OperandSize::Qword)
}

#[test]
fn shr_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RBX, 1113654302, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 171, 30, 4, 97, 66], OperandSize::Qword)
}

#[test]
fn shr_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 235], OperandSize::Word)
}

#[test]
fn shr_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(BX, 175, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 175, 175, 0], OperandSize::Word)
}

#[test]
fn shr_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 237], OperandSize::Dword)
}

#[test]
fn shr_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1555522104, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 172, 151, 56, 98, 183, 92], OperandSize::Dword)
}

#[test]
fn shr_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 237], OperandSize::Qword)
}

#[test]
fn shr_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RAX, 991169788, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 168, 252, 12, 20, 59], OperandSize::Qword)
}

#[test]
fn shr_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(RSI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 238], OperandSize::Qword)
}

#[test]
fn shr_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 44, 206], OperandSize::Qword)
}

#[test]
fn shr_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 234], OperandSize::Word)
}

#[test]
fn shr_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 18, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 105, 18], OperandSize::Word)
}

#[test]
fn shr_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 234], OperandSize::Dword)
}

#[test]
fn shr_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 44, 64], OperandSize::Dword)
}

#[test]
fn shr_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 233], OperandSize::Qword)
}

#[test]
fn shr_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 44, 82], OperandSize::Qword)
}

#[test]
fn shr_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 234], OperandSize::Qword)
}

#[test]
fn shr_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RDI, 3838991, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 175, 15, 148, 58, 0], OperandSize::Qword)
}

#[test]
fn shr_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 238], OperandSize::Word)
}

#[test]
fn shr_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(DI, 125, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 109, 125], OperandSize::Word)
}

#[test]
fn shr_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 236], OperandSize::Dword)
}

#[test]
fn shr_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1555472983, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 44, 93, 87, 162, 182, 92], OperandSize::Dword)
}

#[test]
fn shr_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 234], OperandSize::Qword)
}

#[test]
fn shr_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1267265070, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 44, 189, 46, 238, 136, 75], OperandSize::Qword)
}

#[test]
fn shr_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 238], OperandSize::Word)
}

#[test]
fn shr_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(SI, 21507, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 172, 3, 84], OperandSize::Word)
}

#[test]
fn shr_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 239], OperandSize::Dword)
}

#[test]
fn shr_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(EBX, 595375838, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 171, 222, 182, 124, 35], OperandSize::Dword)
}

#[test]
fn shr_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 238], OperandSize::Qword)
}

#[test]
fn shr_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 41], OperandSize::Qword)
}

#[test]
fn shr_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(Direct(RSP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 236], OperandSize::Qword)
}

#[test]
fn shr_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SHR, operand1: Some(IndirectDisplaced(RDI, 1046874879, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 175, 255, 10, 102, 62], OperandSize::Qword)
}

