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
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 12, Some(OperandSize::Byte), None)), operand2: Some(Literal8(124)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 99, 12, 124], OperandSize::Word)
}

#[test]
fn shl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 225, 88], OperandSize::Dword)
}

#[test]
fn shl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(EDX, Four, 987953224, Some(OperandSize::Byte), None)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 149, 72, 248, 226, 58, 37], OperandSize::Dword)
}

#[test]
fn shl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 100], OperandSize::Qword)
}

#[test]
fn shl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 38, 86], OperandSize::Qword)
}

#[test]
fn shl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 117], OperandSize::Qword)
}

#[test]
fn shl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 35, 80], OperandSize::Qword)
}

#[test]
fn shl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(SP)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 228, 62], OperandSize::Word)
}

#[test]
fn shl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 30309, Some(OperandSize::Word), None)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 162, 101, 118, 111], OperandSize::Word)
}

#[test]
fn shl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DI)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 231, 84], OperandSize::Dword)
}

#[test]
fn shl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 39, 111], OperandSize::Dword)
}

#[test]
fn shl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BX)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 227, 30], OperandSize::Qword)
}

#[test]
fn shl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 36, 64, 126], OperandSize::Qword)
}

#[test]
fn shl_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Word)
}

#[test]
fn shl_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 207, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 161, 207, 0], OperandSize::Word)
}

#[test]
fn shl_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 226], OperandSize::Dword)
}

#[test]
fn shl_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(EAX, 656181444, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 160, 196, 136, 28, 39], OperandSize::Dword)
}

#[test]
fn shl_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 226], OperandSize::Qword)
}

#[test]
fn shl_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(RAX, 288958656, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 160, 192, 40, 57, 17], OperandSize::Qword)
}

#[test]
fn shl_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Qword)
}

#[test]
fn shl_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1802391712, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 164, 126, 160, 80, 110, 107], OperandSize::Qword)
}

#[test]
fn shl_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 228], OperandSize::Word)
}

#[test]
fn shl_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(DI, 26384, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 165, 16, 103], OperandSize::Word)
}

#[test]
fn shl_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 230], OperandSize::Dword)
}

#[test]
fn shl_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1395592306, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 36, 77, 114, 12, 47, 83], OperandSize::Dword)
}

#[test]
fn shl_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 230], OperandSize::Qword)
}

#[test]
fn shl_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 36, 150], OperandSize::Qword)
}

#[test]
fn shl_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 225], OperandSize::Word)
}

#[test]
fn shl_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(BX, 173, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 167, 173, 0], OperandSize::Word)
}

#[test]
fn shl_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 229], OperandSize::Dword)
}

#[test]
fn shl_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(ESI, 1126023809, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 166, 129, 194, 29, 67], OperandSize::Dword)
}

#[test]
fn shl_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 227], OperandSize::Qword)
}

#[test]
fn shl_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 36, 208], OperandSize::Qword)
}

#[test]
fn shl_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Word)
}

#[test]
fn shl_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36], OperandSize::Word)
}

#[test]
fn shl_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Dword)
}

#[test]
fn shl_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1528251317, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 164, 114, 181, 67, 23, 91], OperandSize::Dword)
}

#[test]
fn shl_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Qword)
}

#[test]
fn shl_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RCX, Two, 483147433, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36, 77, 169, 62, 204, 28], OperandSize::Qword)
}

#[test]
fn shl_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Qword)
}

#[test]
fn shl_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectScaledDisplaced(RDX, Four, 647429458, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36, 149, 82, 253, 150, 38], OperandSize::Qword)
}

#[test]
fn shl_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 230], OperandSize::Word)
}

#[test]
fn shl_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 36], OperandSize::Word)
}

#[test]
fn shl_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 227], OperandSize::Dword)
}

#[test]
fn shl_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 33], OperandSize::Dword)
}

#[test]
fn shl_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 230], OperandSize::Qword)
}

#[test]
fn shl_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SHL, operand1: Some(IndirectDisplaced(RBX, 1621600589, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 163, 77, 169, 167, 96], OperandSize::Qword)
}

