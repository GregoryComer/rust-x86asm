use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 210, 72], OperandSize::Word)
}

#[test]
fn rcl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(BP, 39, Some(OperandSize::Byte), None)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 86, 39, 38], OperandSize::Word)
}

#[test]
fn rcl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 210, 10], OperandSize::Dword)
}

#[test]
fn rcl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 2076244167, Some(OperandSize::Byte), None)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 148, 211, 199, 248, 192, 123, 56], OperandSize::Dword)
}

#[test]
fn rcl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CL)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 209, 113], OperandSize::Qword)
}

#[test]
fn rcl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 20, 137, 90], OperandSize::Qword)
}

#[test]
fn rcl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 211, 34], OperandSize::Qword)
}

#[test]
fn rcl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 20, 73, 75], OperandSize::Qword)
}

#[test]
fn rcl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DI)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 215, 28], OperandSize::Word)
}

#[test]
fn rcl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 20, 4], OperandSize::Word)
}

#[test]
fn rcl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(SP)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 212, 43], OperandSize::Dword)
}

#[test]
fn rcl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 20, 191, 24], OperandSize::Dword)
}

#[test]
fn rcl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DI)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 215, 20], OperandSize::Qword)
}

#[test]
fn rcl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 20, 145, 29], OperandSize::Qword)
}

#[test]
fn rcl_15() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 214, 55], OperandSize::Word)
}

#[test]
fn rcl_16() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 5450, Some(OperandSize::Dword), None)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 144, 74, 21, 116], OperandSize::Word)
}

#[test]
fn rcl_17() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 215, 88], OperandSize::Dword)
}

#[test]
fn rcl_18() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 20, 207, 76], OperandSize::Dword)
}

#[test]
fn rcl_19() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 214, 44], OperandSize::Qword)
}

#[test]
fn rcl_20() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RBX, Four, 911595063, Some(OperandSize::Dword), None)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 20, 157, 55, 214, 85, 54, 52], OperandSize::Qword)
}

#[test]
fn rcl_21() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RBX)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 211, 79], OperandSize::Qword)
}

#[test]
fn rcl_22() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1985309097, Some(OperandSize::Qword), None)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 148, 126, 169, 105, 85, 118, 25], OperandSize::Qword)
}

#[test]
fn rcl_23() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 210], OperandSize::Word)
}

#[test]
fn rcl_24() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 182, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 146, 182, 0], OperandSize::Word)
}

#[test]
fn rcl_25() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 210], OperandSize::Dword)
}

#[test]
fn rcl_26() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(EDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 18], OperandSize::Dword)
}

#[test]
fn rcl_27() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 211], OperandSize::Qword)
}

#[test]
fn rcl_28() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RBX, Two, 2044200455, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 20, 93, 7, 6, 216, 121], OperandSize::Qword)
}

#[test]
fn rcl_29() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 210], OperandSize::Qword)
}

#[test]
fn rcl_30() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RDI, 1153523184, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 151, 240, 93, 193, 68], OperandSize::Qword)
}

#[test]
fn rcl_31() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 213], OperandSize::Word)
}

#[test]
fn rcl_32() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 32143, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 145, 143, 125], OperandSize::Word)
}

#[test]
fn rcl_33() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 211], OperandSize::Dword)
}

#[test]
fn rcl_34() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1497201149, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 20, 221, 253, 121, 61, 89], OperandSize::Dword)
}

#[test]
fn rcl_35() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 209], OperandSize::Qword)
}

#[test]
fn rcl_36() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 18], OperandSize::Qword)
}

#[test]
fn rcl_37() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 215], OperandSize::Word)
}

#[test]
fn rcl_38() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 40, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 82, 40], OperandSize::Word)
}

#[test]
fn rcl_39() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 209], OperandSize::Dword)
}

#[test]
fn rcl_40() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(ESI, 627280628, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 150, 244, 138, 99, 37], OperandSize::Dword)
}

#[test]
fn rcl_41() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 209], OperandSize::Qword)
}

#[test]
fn rcl_42() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 20, 203], OperandSize::Qword)
}

#[test]
fn rcl_43() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RCX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 209], OperandSize::Qword)
}

#[test]
fn rcl_44() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 20, 207], OperandSize::Qword)
}

#[test]
fn rcl_45() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 209], OperandSize::Word)
}

#[test]
fn rcl_46() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 85, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 83, 85], OperandSize::Word)
}

#[test]
fn rcl_47() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 210], OperandSize::Dword)
}

#[test]
fn rcl_48() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1670432898, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 148, 72, 130, 200, 144, 99], OperandSize::Dword)
}

#[test]
fn rcl_49() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 211], OperandSize::Qword)
}

#[test]
fn rcl_50() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RSI, 478064714, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 150, 74, 176, 126, 28], OperandSize::Qword)
}

#[test]
fn rcl_51() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 211], OperandSize::Qword)
}

#[test]
fn rcl_52() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1731392765, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 148, 154, 253, 244, 50, 103], OperandSize::Qword)
}

#[test]
fn rcl_53() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 215], OperandSize::Word)
}

#[test]
fn rcl_54() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Memory(2301, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 22, 253, 8], OperandSize::Word)
}

#[test]
fn rcl_55() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 211], OperandSize::Dword)
}

#[test]
fn rcl_56() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1179901574, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 20, 197, 134, 222, 83, 70], OperandSize::Dword)
}

#[test]
fn rcl_57() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 212], OperandSize::Qword)
}

#[test]
fn rcl_58() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 2125983400, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 148, 153, 168, 238, 183, 126], OperandSize::Qword)
}

#[test]
fn rcl_59() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 211], OperandSize::Word)
}

#[test]
fn rcl_60() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 23], OperandSize::Word)
}

#[test]
fn rcl_61() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 213], OperandSize::Dword)
}

#[test]
fn rcl_62() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 16], OperandSize::Dword)
}

#[test]
fn rcl_63() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 209], OperandSize::Qword)
}

#[test]
fn rcl_64() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1588193280, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 148, 195, 0, 232, 169, 94], OperandSize::Qword)
}

#[test]
fn rcl_65() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 210], OperandSize::Qword)
}

#[test]
fn rcl_66() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 20, 202], OperandSize::Qword)
}

