use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 210, 89], OperandSize::Word)
}

#[test]
fn rcl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(DI, 99, Some(OperandSize::Byte), None)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 85, 99, 82], OperandSize::Word)
}

#[test]
fn rcl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 211, 98], OperandSize::Dword)
}

#[test]
fn rcl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 20, 176, 14], OperandSize::Dword)
}

#[test]
fn rcl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 211, 17], OperandSize::Qword)
}

#[test]
fn rcl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 20, 223, 61], OperandSize::Qword)
}

#[test]
fn rcl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 211, 18], OperandSize::Qword)
}

#[test]
fn rcl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RSI, 602098926, Some(OperandSize::Byte), None)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 150, 238, 76, 227, 35, 61], OperandSize::Qword)
}

#[test]
fn rcl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BX)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 211, 38], OperandSize::Word)
}

#[test]
fn rcl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(SI, 107, Some(OperandSize::Word), None)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 84, 107, 122], OperandSize::Word)
}

#[test]
fn rcl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(SP)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 212, 105], OperandSize::Dword)
}

#[test]
fn rcl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(EDI, Four, 803560522, Some(OperandSize::Word), None)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 20, 189, 74, 92, 229, 47, 10], OperandSize::Dword)
}

#[test]
fn rcl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(SP)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 212, 39], OperandSize::Qword)
}

#[test]
fn rcl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 22, 48], OperandSize::Qword)
}

#[test]
fn rcl_15() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 211, 106], OperandSize::Word)
}

#[test]
fn rcl_16() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 16, 126], OperandSize::Word)
}

#[test]
fn rcl_17() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 213, 43], OperandSize::Dword)
}

#[test]
fn rcl_18() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 18, 125], OperandSize::Dword)
}

#[test]
fn rcl_19() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 213, 91], OperandSize::Qword)
}

#[test]
fn rcl_20() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1896091737, Some(OperandSize::Dword), None)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 20, 253, 89, 16, 4, 113, 120], OperandSize::Qword)
}

#[test]
fn rcl_21() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RDX)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 210, 59], OperandSize::Qword)
}

#[test]
fn rcl_22() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1722651167, Some(OperandSize::Qword), None)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 148, 191, 31, 146, 173, 102, 44], OperandSize::Qword)
}

#[test]
fn rcl_23() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 211], OperandSize::Word)
}

#[test]
fn rcl_24() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 17669, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 144, 5, 69], OperandSize::Word)
}

#[test]
fn rcl_25() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 211], OperandSize::Dword)
}

#[test]
fn rcl_26() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 22], OperandSize::Dword)
}

#[test]
fn rcl_27() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 210], OperandSize::Qword)
}

#[test]
fn rcl_28() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RDI, 58930855, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 151, 167, 54, 131, 3], OperandSize::Qword)
}

#[test]
fn rcl_29() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 210], OperandSize::Qword)
}

#[test]
fn rcl_30() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 1206949209, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 148, 65, 89, 149, 240, 71], OperandSize::Qword)
}

#[test]
fn rcl_31() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 210], OperandSize::Word)
}

#[test]
fn rcl_32() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 20005, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 147, 37, 78], OperandSize::Word)
}

#[test]
fn rcl_33() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 209], OperandSize::Dword)
}

#[test]
fn rcl_34() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 23], OperandSize::Dword)
}

#[test]
fn rcl_35() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 215], OperandSize::Qword)
}

#[test]
fn rcl_36() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 23], OperandSize::Qword)
}

#[test]
fn rcl_37() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 212], OperandSize::Word)
}

#[test]
fn rcl_38() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 17], OperandSize::Word)
}

#[test]
fn rcl_39() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 212], OperandSize::Dword)
}

#[test]
fn rcl_40() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1420190348, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 20, 197, 140, 98, 166, 84], OperandSize::Dword)
}

#[test]
fn rcl_41() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 212], OperandSize::Qword)
}

#[test]
fn rcl_42() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 16], OperandSize::Qword)
}

#[test]
fn rcl_43() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 210], OperandSize::Qword)
}

#[test]
fn rcl_44() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1642095366, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 148, 193, 6, 99, 224, 97], OperandSize::Qword)
}

#[test]
fn rcl_45() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 211], OperandSize::Word)
}

#[test]
fn rcl_46() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 23], OperandSize::Word)
}

#[test]
fn rcl_47() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 209], OperandSize::Dword)
}

#[test]
fn rcl_48() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(EBX, Two, 2141757889, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 20, 93, 193, 161, 168, 127], OperandSize::Dword)
}

#[test]
fn rcl_49() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 211], OperandSize::Qword)
}

#[test]
fn rcl_50() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 20, 147], OperandSize::Qword)
}

#[test]
fn rcl_51() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 211], OperandSize::Qword)
}

#[test]
fn rcl_52() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RAX, 18191877, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 144, 5, 150, 21, 1], OperandSize::Qword)
}

#[test]
fn rcl_53() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 210], OperandSize::Word)
}

#[test]
fn rcl_54() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 128, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 145, 128, 0], OperandSize::Word)
}

#[test]
fn rcl_55() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 211], OperandSize::Dword)
}

#[test]
fn rcl_56() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 2010091932, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 20, 213, 156, 145, 207, 119], OperandSize::Dword)
}

#[test]
fn rcl_57() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 209], OperandSize::Qword)
}

#[test]
fn rcl_58() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1529150788, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 20, 181, 68, 253, 36, 91], OperandSize::Qword)
}

#[test]
fn rcl_59() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 214], OperandSize::Word)
}

#[test]
fn rcl_60() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(SI, 4876, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 148, 12, 19], OperandSize::Word)
}

#[test]
fn rcl_61() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 215], OperandSize::Dword)
}

#[test]
fn rcl_62() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 17], OperandSize::Dword)
}

#[test]
fn rcl_63() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 211], OperandSize::Qword)
}

#[test]
fn rcl_64() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RDX, Four, 733577229, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 20, 149, 13, 128, 185, 43], OperandSize::Qword)
}

#[test]
fn rcl_65() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 215], OperandSize::Qword)
}

#[test]
fn rcl_66() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 17], OperandSize::Qword)
}

