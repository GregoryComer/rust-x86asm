use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 210, 121], OperandSize::Word)
}

#[test]
fn rcl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(SI, 10629, Some(OperandSize::Byte), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 148, 133, 41, 104], OperandSize::Word)
}

#[test]
fn rcl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 211, 40], OperandSize::Dword)
}

#[test]
fn rcl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(EAX, 1779930493, Some(OperandSize::Byte), None)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 144, 125, 149, 23, 106, 101], OperandSize::Dword)
}

#[test]
fn rcl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CL)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 209, 31], OperandSize::Qword)
}

#[test]
fn rcl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 880559642, Some(OperandSize::Byte), None)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 148, 155, 26, 70, 124, 52, 76], OperandSize::Qword)
}

#[test]
fn rcl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CL)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 209, 90], OperandSize::Qword)
}

#[test]
fn rcl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RSI, 1264008947, Some(OperandSize::Byte), None)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 150, 243, 62, 87, 75, 90], OperandSize::Qword)
}

#[test]
fn rcl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DI)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 215, 91], OperandSize::Word)
}

#[test]
fn rcl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 246, Some(OperandSize::Word), None)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 147, 246, 0, 19], OperandSize::Word)
}

#[test]
fn rcl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(SP)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 212, 27], OperandSize::Dword)
}

#[test]
fn rcl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 22, 68], OperandSize::Dword)
}

#[test]
fn rcl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DX)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 210, 94], OperandSize::Qword)
}

#[test]
fn rcl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 614723728, Some(OperandSize::Word), None)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 148, 129, 144, 240, 163, 36, 117], OperandSize::Qword)
}

#[test]
fn rcl_15() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 209, 58], OperandSize::Word)
}

#[test]
fn rcl_16() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 61, Some(OperandSize::Dword), None)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 81, 61, 56], OperandSize::Word)
}

#[test]
fn rcl_17() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 213, 69], OperandSize::Dword)
}

#[test]
fn rcl_18() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 20, 242, 2], OperandSize::Dword)
}

#[test]
fn rcl_19() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 211, 14], OperandSize::Qword)
}

#[test]
fn rcl_20() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 20, 208, 110], OperandSize::Qword)
}

#[test]
fn rcl_21() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RBP)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 213, 113], OperandSize::Qword)
}

#[test]
fn rcl_22() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 1876008382, Some(OperandSize::Qword), None)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 148, 94, 190, 157, 209, 111, 73], OperandSize::Qword)
}

#[test]
fn rcl_23() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 209], OperandSize::Word)
}

#[test]
fn rcl_24() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 29816, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 146, 120, 116], OperandSize::Word)
}

#[test]
fn rcl_25() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 209], OperandSize::Dword)
}

#[test]
fn rcl_26() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 20, 137], OperandSize::Dword)
}

#[test]
fn rcl_27() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 210], OperandSize::Qword)
}

#[test]
fn rcl_28() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 20, 186], OperandSize::Qword)
}

#[test]
fn rcl_29() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 210], OperandSize::Qword)
}

#[test]
fn rcl_30() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RAX, 348933477, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 144, 101, 77, 204, 20], OperandSize::Qword)
}

#[test]
fn rcl_31() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 215], OperandSize::Word)
}

#[test]
fn rcl_32() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 16], OperandSize::Word)
}

#[test]
fn rcl_33() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 213], OperandSize::Dword)
}

#[test]
fn rcl_34() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 22], OperandSize::Dword)
}

#[test]
fn rcl_35() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 214], OperandSize::Qword)
}

#[test]
fn rcl_36() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 19], OperandSize::Qword)
}

#[test]
fn rcl_37() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 215], OperandSize::Word)
}

#[test]
fn rcl_38() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 11988, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 147, 212, 46], OperandSize::Word)
}

#[test]
fn rcl_39() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 213], OperandSize::Dword)
}

#[test]
fn rcl_40() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(EDI, 1477014860, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 151, 76, 117, 9, 88], OperandSize::Dword)
}

#[test]
fn rcl_41() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 215], OperandSize::Qword)
}

#[test]
fn rcl_42() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RSI, Four, 353858363, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 20, 181, 59, 115, 23, 21], OperandSize::Qword)
}

#[test]
fn rcl_43() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 213], OperandSize::Qword)
}

#[test]
fn rcl_44() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1686998224, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 20, 205, 208, 140, 141, 100], OperandSize::Qword)
}

#[test]
fn rcl_45() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 210], OperandSize::Word)
}

#[test]
fn rcl_46() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 21], OperandSize::Word)
}

#[test]
fn rcl_47() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 211], OperandSize::Dword)
}

#[test]
fn rcl_48() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 830477708, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 148, 248, 140, 21, 128, 49], OperandSize::Dword)
}

#[test]
fn rcl_49() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 210], OperandSize::Qword)
}

#[test]
fn rcl_50() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RDX, 1982398827, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 146, 107, 1, 41, 118], OperandSize::Qword)
}

#[test]
fn rcl_51() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 211], OperandSize::Qword)
}

#[test]
fn rcl_52() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 826452134, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 148, 216, 166, 168, 66, 49], OperandSize::Qword)
}

#[test]
fn rcl_53() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 214], OperandSize::Word)
}

#[test]
fn rcl_54() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 21], OperandSize::Word)
}

#[test]
fn rcl_55() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(DI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 215], OperandSize::Dword)
}

#[test]
fn rcl_56() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1322088851, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 148, 190, 147, 121, 205, 78], OperandSize::Dword)
}

#[test]
fn rcl_57() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 213], OperandSize::Qword)
}

#[test]
fn rcl_58() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledDisplaced(RDI, Two, 397714202, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 20, 125, 26, 163, 180, 23], OperandSize::Qword)
}

#[test]
fn rcl_59() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 209], OperandSize::Word)
}

#[test]
fn rcl_60() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 64, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 80, 64], OperandSize::Word)
}

#[test]
fn rcl_61() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(EBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 211], OperandSize::Dword)
}

#[test]
fn rcl_62() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 17], OperandSize::Dword)
}

#[test]
fn rcl_63() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 212], OperandSize::Qword)
}

#[test]
fn rcl_64() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectDisplaced(RDI, 1641184277, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 151, 21, 124, 210, 97], OperandSize::Qword)
}

#[test]
fn rcl_65() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(Direct(RBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 211], OperandSize::Qword)
}

#[test]
fn rcl_66() {
    run_test(&Instruction { mnemonic: Mnemonic::RCL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 907863646, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 148, 118, 94, 230, 28, 54], OperandSize::Qword)
}

