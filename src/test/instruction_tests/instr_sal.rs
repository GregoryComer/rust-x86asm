use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sal_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 226, 122], OperandSize::Word)
}

#[test]
fn sal_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(BP, 159, Some(OperandSize::Byte), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 166, 159, 0, 110], OperandSize::Word)
}

#[test]
fn sal_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 225, 51], OperandSize::Dword)
}

#[test]
fn sal_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(EAX, Two, 674803693, Some(OperandSize::Byte), None)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 69, 237, 175, 56, 40, 94], OperandSize::Dword)
}

#[test]
fn sal_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 226, 110], OperandSize::Qword)
}

#[test]
fn sal_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(RDX, 1032239727, Some(OperandSize::Byte), None)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 162, 111, 186, 134, 61, 120], OperandSize::Qword)
}

#[test]
fn sal_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 95], OperandSize::Qword)
}

#[test]
fn sal_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RBX, Four, 545476167, Some(OperandSize::Byte), None)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 157, 71, 78, 131, 32, 117], OperandSize::Qword)
}

#[test]
fn sal_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DX)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 226, 28], OperandSize::Word)
}

#[test]
fn sal_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 146, Some(OperandSize::Word), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 162, 146, 0, 81], OperandSize::Word)
}

#[test]
fn sal_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CX)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 225, 9], OperandSize::Dword)
}

#[test]
fn sal_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 568504967, Some(OperandSize::Word), None)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 164, 222, 135, 178, 226, 33, 60], OperandSize::Dword)
}

#[test]
fn sal_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CX)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 225, 23], OperandSize::Qword)
}

#[test]
fn sal_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 36, 183, 12], OperandSize::Qword)
}

#[test]
fn sal_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EDX)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 226, 52], OperandSize::Word)
}

#[test]
fn sal_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 9945, Some(OperandSize::Dword), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 161, 217, 38, 35], OperandSize::Word)
}

#[test]
fn sal_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 228, 38], OperandSize::Dword)
}

#[test]
fn sal_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 38, 43], OperandSize::Dword)
}

#[test]
fn sal_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 225, 67], OperandSize::Qword)
}

#[test]
fn sal_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1586922023, Some(OperandSize::Dword), None)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 164, 254, 39, 130, 150, 94, 73], OperandSize::Qword)
}

#[test]
fn sal_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RCX)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 225, 10], OperandSize::Qword)
}

#[test]
fn sal_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 36, 123, 94], OperandSize::Qword)
}

#[test]
fn sal_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Word)
}

#[test]
fn sal_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(DI, 18814, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 165, 126, 73], OperandSize::Word)
}

#[test]
fn sal_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 226], OperandSize::Dword)
}

#[test]
fn sal_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 39], OperandSize::Dword)
}

#[test]
fn sal_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Qword)
}

#[test]
fn sal_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 33], OperandSize::Qword)
}

#[test]
fn sal_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 226], OperandSize::Qword)
}

#[test]
fn sal_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 38], OperandSize::Qword)
}

#[test]
fn sal_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 229], OperandSize::Word)
}

#[test]
fn sal_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 32, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 97, 32], OperandSize::Word)
}

#[test]
fn sal_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 229], OperandSize::Dword)
}

#[test]
fn sal_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 36, 192], OperandSize::Dword)
}

#[test]
fn sal_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 230], OperandSize::Qword)
}

#[test]
fn sal_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 34], OperandSize::Qword)
}

#[test]
fn sal_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 227], OperandSize::Word)
}

#[test]
fn sal_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(BX, 178, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 167, 178, 0], OperandSize::Word)
}

#[test]
fn sal_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 231], OperandSize::Dword)
}

#[test]
fn sal_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(EBX, Four, 1933752134, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 36, 157, 70, 183, 66, 115], OperandSize::Dword)
}

#[test]
fn sal_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 227], OperandSize::Qword)
}

#[test]
fn sal_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 36, 179], OperandSize::Qword)
}

#[test]
fn sal_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RSI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 230], OperandSize::Qword)
}

#[test]
fn sal_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(RDI, 1084572436, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 167, 20, 67, 165, 64], OperandSize::Qword)
}

#[test]
fn sal_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 226], OperandSize::Word)
}

#[test]
fn sal_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(BX, 162, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 167, 162, 0], OperandSize::Word)
}

#[test]
fn sal_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Dword)
}

#[test]
fn sal_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 905400944, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 164, 128, 112, 82, 247, 53], OperandSize::Dword)
}

#[test]
fn sal_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Qword)
}

#[test]
fn sal_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 33], OperandSize::Qword)
}

#[test]
fn sal_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Qword)
}

#[test]
fn sal_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36, 138], OperandSize::Qword)
}

#[test]
fn sal_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 228], OperandSize::Word)
}

#[test]
fn sal_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 37], OperandSize::Word)
}

#[test]
fn sal_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 227], OperandSize::Dword)
}

#[test]
fn sal_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 39], OperandSize::Dword)
}

#[test]
fn sal_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 225], OperandSize::Qword)
}

#[test]
fn sal_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 36, 247], OperandSize::Qword)
}

#[test]
fn sal_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 225], OperandSize::Word)
}

#[test]
fn sal_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 228, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 162, 228, 0], OperandSize::Word)
}

#[test]
fn sal_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 228], OperandSize::Dword)
}

#[test]
fn sal_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(ECX, 681915895, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 161, 247, 53, 165, 40], OperandSize::Dword)
}

#[test]
fn sal_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 228], OperandSize::Qword)
}

#[test]
fn sal_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 7265283, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 36, 245, 3, 220, 110, 0], OperandSize::Qword)
}

#[test]
fn sal_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RCX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 225], OperandSize::Qword)
}

#[test]
fn sal_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(RCX, 335945695, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 161, 223, 31, 6, 20], OperandSize::Qword)
}

